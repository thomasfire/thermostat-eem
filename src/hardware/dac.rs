use defmt::Format;

///! Thermostat DAC driver
///!
///! This file contains the driver for the 4 Thermostat DAC output channels.
///! To convert a 18 bit word into an analog current Thermostat uses a DAC to
///! convert the word into a voltage and a subsequent TEC driver IC that produces
///! a current proportional to the DAC voltage.
///!
///! The 4 channel DAC ICs share an SPI bus and are addressed using individual "sync"
///! signals, similar to a chip select signal.
///! The TEC driver ICs feature a shutdown mode controlled by a shutdown signal and
///! current limits controlled by another input voltage. The shutdown signal is aggregated
///! into the DAC driver using a gpio, while the current limits make use of the Thermostat
///! PWM driver.
///! DAC datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/AD5680.pdf
///! TEC driver datasheet: https://datasheets.maximintegrated.com/en/ds/MAX1968-MAX1969.pdf
///!
use super::hal::{
    gpio::{gpioc::*, gpiog::*, Alternate, Output, PushPull},
    hal::blocking::spi::Write,
    prelude::*,
    rcc::{rec, CoreClocks},
    spi::{Enabled, NoMiso, Spi, MODE_1},
    stm32::SPI3,
    time::MegaHertz,
};

use super::OutputChannelIdx;

// Note: 30MHz clock valid according to DAC datasheet. This lead to spurious RxFIFO overruns on the STM side when probing the spi clock with a scope probe.
const SPI_CLOCK: MegaHertz = MegaHertz::MHz(8);

// DAC and PWM shared constants
pub const R_SENSE: f32 = 0.05; // TEC current sense resistor
pub const VREF_TEC: f32 = 1.5; // TEC driver reference voltage

/// DAC value out of bounds error.
#[derive(Debug)]
pub enum Error {
    Bounds,
}

/// A type representing a DAC sample.
#[derive(Copy, Clone, Debug, Format)]
pub struct DacCode(u32);
impl DacCode {
    // DAC constants
    const MAX_DAC_WORD: i32 = 1 << 20; // maximum DAC dataword (exclusive) plus 2 bit due to interface alignment
    const VREF_DAC: f32 = 3.0; // DAC reference voltage
    pub const MAX_CURRENT: f32 = ((((DacCode::MAX_DAC_WORD - 1) as f32 * DacCode::VREF_DAC)
        / DacCode::MAX_DAC_WORD as f32)
        - VREF_TEC)
        / (10.0 * R_SENSE);
}

impl TryFrom<f32> for DacCode {
    type Error = Error;
    /// Convert an f32 representing a current int the corresponding DAC output code.
    fn try_from(current: f32) -> Result<DacCode, Error> {
        // Current to DAC word conversion
        let ctli_voltage = (current * 10.0 * R_SENSE) + VREF_TEC;
        let dac_code = (ctli_voltage * (DacCode::MAX_DAC_WORD as f32 / DacCode::VREF_DAC)) as i32;

        if !(0..DacCode::MAX_DAC_WORD).contains(&dac_code) {
            return Err(Error::Bounds);
        };

        Ok(Self(dac_code as u32))
    }
}

impl From<DacCode> for u32 {
    fn from(code: DacCode) -> u32 {
        code.0
    }
}

/// DAC gpio pins.
///
/// sync<n> - DAC IC adressing signals
/// * <n> specifies Thermostat output channel
#[allow(clippy::type_complexity)]
pub struct DacPins {
    pub sync: (
        PG3<Output<PushPull>>,
        PG2<Output<PushPull>>,
        PG1<Output<PushPull>>,
        PG0<Output<PushPull>>,
    ),
}

/// DAC driver struct containing the SPI bus and the gpio pins.
pub struct Dac {
    spi: Spi<SPI3, Enabled, u8>,
    pins: DacPins,
}

impl Dac {
    /// Construct a new DAC driver for all Thermostat output channels.
    ///
    /// # Args
    /// * `clocks` - Reference to CoreClocks
    /// * `spi3_rec` - Peripheral Reset and Enable Control for SPI3
    /// * `spi3` - SPI3 peripheral
    /// * `sck` - SPI3 sck pin
    /// * `mosi` - SPI3 mosi pin
    /// * `pins` - DAC sync pins.
    pub fn new(
        clocks: &CoreClocks,
        spi3_rec: rec::Spi3,
        spi3: SPI3,
        sck: PC10<Alternate<6>>,
        mosi: PC12<Alternate<6>>,
        pins: DacPins,
    ) -> Self {
        let spi = spi3.spi(
            (sck, NoMiso, mosi),
            MODE_1,
            SPI_CLOCK.convert(),
            spi3_rec,
            clocks,
        );

        let mut dac = Dac { spi, pins };

        dac.pins.sync.0.set_high();
        dac.pins.sync.1.set_high();
        dac.pins.sync.2.set_high();
        dac.pins.sync.3.set_high();

        // default to zero current
        for i in 0..4 {
            let ch = OutputChannelIdx::try_from(i).unwrap();
            dac.set(ch, (0.0).try_into().unwrap());
        }
        dac
    }

    /// Set the DAC output to on a channel.
    ///
    /// # Args
    /// * `ch` - Thermostat output channel
    /// * `dac_code` - dac output code to transfer
    pub fn set(&mut self, ch: OutputChannelIdx, dac_code: DacCode) {
        let buf = &(dac_code.0).to_be_bytes()[1..];

        match ch {
            OutputChannelIdx::Zero => {
                self.pins.sync.0.set_low();
                // 24 bit write. 4 MSB are zero and 2 LSB are ignored for a 18 bit DAC output.
                self.spi.write(buf).unwrap();
                self.pins.sync.0.set_high();
            }
            OutputChannelIdx::One => {
                self.pins.sync.1.set_low();
                self.spi.write(buf).unwrap();
                self.pins.sync.1.set_high();
            }
            OutputChannelIdx::Two => {
                self.pins.sync.2.set_low();
                self.spi.write(buf).unwrap();
                self.pins.sync.2.set_high();
            }
            OutputChannelIdx::Three => {
                self.pins.sync.3.set_low();
                self.spi.write(buf).unwrap();
                self.pins.sync.3.set_high();
            }
        }
    }
}