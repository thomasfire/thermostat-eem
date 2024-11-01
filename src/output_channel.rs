//! # Thermostat_EEM IIR wrapper.
//!

use crate::{hardware::pwm::Pwm, DacCode};
use idsp::iir;
use miniconf::{Leaf, Tree};
use num_traits::Float;

#[derive(Copy, Clone, Debug, Tree)]
pub struct Pid {
    /// Integral gain
    ///
    /// Units: output/input per second
    pub ki: Leaf<f32>,
    /// Proportional gain
    ///
    /// Note that this is the sign reference for all gains and limits
    ///
    /// Units: output/input
    pub kp: Leaf<f32>,
    /// Derivative gain
    ///
    /// Units: output/input*second
    pub kd: Leaf<f32>,
    /// Integral gain limit
    ///
    /// Units: output/input
    pub li: Leaf<f32>,
    /// Derivative gain limit
    ///
    /// Units: output/input
    pub ld: Leaf<f32>,
    /// Setpoint
    ///
    /// Units: input
    pub setpoint: Leaf<f32>,
    /// Output lower limit
    ///
    /// Units: output
    pub min: Leaf<f32>,
    /// Output upper limit
    ///
    /// Units: output
    pub max: Leaf<f32>,
}

impl Default for Pid {
    fn default() -> Self {
        Self {
            ki: 0.0.into(),
            kp: 0.0.into(), // positive default
            kd: 0.0.into(),
            li: f32::INFINITY.into(),
            ld: f32::INFINITY.into(),
            setpoint: 25.0.into(),
            min: f32::NEG_INFINITY.into(),
            max: f32::INFINITY.into(),
        }
    }
}

impl TryFrom<Pid> for iir::Biquad<f64> {
    type Error = iir::PidError;
    fn try_from(value: Pid) -> Result<Self, Self::Error> {
        let mut biquad: iir::Biquad<f64> = iir::Pid::<f64>::default()
            .period(1.0 / 1007.0) // ADC sample rate (ODR) including zero-oder-holds
            .gain(iir::Action::Ki, value.ki.copysign(*value.kp) as _)
            .gain(iir::Action::Kp, *value.kp as _)
            .gain(iir::Action::Kd, value.kd.copysign(*value.kp) as _)
            .limit(
                iir::Action::Ki,
                if value.li.is_finite() {
                    *value.li
                } else {
                    f32::INFINITY
                }
                .copysign(*value.kp) as _,
            )
            .limit(
                iir::Action::Kd,
                if value.ld.is_finite() {
                    *value.ld
                } else {
                    f32::INFINITY
                }
                .copysign(*value.kp) as _,
            )
            .build()?
            .into();
        biquad.set_input_offset(-*value.setpoint as _);
        biquad.set_min(if value.min.is_finite() {
            *value.min
        } else {
            f32::NEG_INFINITY
        } as _);
        biquad.set_max(if value.max.is_finite() {
            *value.max
        } else {
            f32::INFINITY
        } as _);
        Ok(biquad)
    }
}

#[derive(
    Copy, Clone, Default, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum State {
    /// Active TEC driver and Biquad/PID.
    On,
    /// Hold the output.
    Hold,
    /// Disables the TEC driver. This implies "hold".
    #[default]
    Off,
}

#[derive(Copy, Clone, Debug, Tree)]
pub struct OutputChannel {
    pub state: Leaf<State>,

    /// Maximum absolute (positive and negative) TEC voltage in volt.
    /// These will be clamped to the maximum of 4.3 V.
    ///
    /// # Value
    /// 0.0 to 4.3
    pub voltage_limit: Leaf<f32>,

    /// PID/Biquad/IIR filter parameters
    ///
    /// The y limits will be clamped to the maximum output current of +-3 A.
    pub pid: Pid,

    #[tree(skip)]
    pub iir: iir::Biquad<f64>,

    /// Thermostat input channel weights. Each input of an enabled input channel
    /// is multiplied by its weight and the accumulated output is fed into the IIR.
    /// The weights will be internally normalized to one (sum of the absolute values)
    /// if they are not all zero.
    pub weights: Leaf<[[f32; 4]; 4]>,
}

impl Default for OutputChannel {
    fn default() -> Self {
        Self {
            state: State::Off.into(),
            voltage_limit: Pwm::MAX_VOLTAGE_LIMIT.into(),
            pid: Default::default(),
            iir: Default::default(),
            weights: Default::default(),
        }
    }
}

impl OutputChannel {
    /// compute weighted iir input, iir state and return the new output
    pub fn update(&mut self, temperatures: &[[f64; 4]; 4], iir_state: &mut [f64; 4]) -> f64 {
        let temperature = temperatures
            .as_flattened()
            .iter()
            .zip(self.weights.as_flattened().iter())
            .map(|(t, w)| t * *w as f64)
            .sum();
        let iir = if *self.state == State::On {
            &self.iir
        } else {
            &iir::Biquad::HOLD
        };
        iir.update(iir_state, temperature)
    }

    /// Performs finalization of the output_channel miniconf settings:
    /// - Clamping of the limits
    /// - Normalization of the weights
    ///
    /// Returns the current limits.
    pub fn finalize_settings(&mut self) {
        if let Ok(iir) = self.pid.try_into() {
            self.iir = iir;
        } else {
            log::info!("Pid build failure, update not applied.");
        }
        let range = DacCode::MAX_CURRENT.min(Pwm::MAX_CURRENT_LIMIT);
        self.iir
            .set_max(self.iir.max().clamp(-range as _, range as _));
        self.iir
            .set_min(self.iir.min().clamp(-range as _, range as _));
        *self.voltage_limit = (*self.voltage_limit).clamp(0.0, Pwm::MAX_VOLTAGE_LIMIT);
        let divisor: f32 = self.weights.iter().flatten().map(|w| w.abs()).sum();
        // Note: The weights which are not 'None' should always affect an enabled channel and therefore count for normalization.
        if divisor != 0.0 {
            let n = divisor.recip();
            for w in self.weights.as_flattened_mut().iter_mut() {
                *w *= n;
            }
        }
    }

    pub fn current_limits(&self) -> [f32; 2] {
        [
            // give 5% extra headroom for PWM current limits
            // [Pwm::MAX_CURRENT_LIMIT] + 5% is still below 100% duty cycle for the PWM limits and therefore OK.
            // Might not be OK for a different shunt resistor or different PWM setup.
            (self.iir.max() as f32 + 0.05 * Pwm::MAX_CURRENT_LIMIT).max(0.),
            (self.iir.min() as f32 - 0.05 * Pwm::MAX_CURRENT_LIMIT).min(0.),
        ]
    }
}
