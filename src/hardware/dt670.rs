/// DT-670 Silicon Diode curve
///
/// https://www.lakeshore.com/docs/default-source/product-downloads/catalog/lstc_dt670_l.pdf
///
/// (T (K), voltage (V), dV/dT (mV/K))
#[allow(clippy::excessive_precision)]
pub const CURVE: [(f32, f32, f32); 144] = [
    (1.4, 1.644290, -12.5),
    (1.5, 1.642990, -13.6),
    (1.6, 1.641570, -14.8),
    (1.7, 1.640030, -16.0),
    (1.8, 1.638370, -17.1),
    (1.9, 1.636600, -18.3),
    (2.0, 1.634720, -19.3),
    (2.1, 1.632740, -20.3),
    (2.2, 1.630670, -21.1),
    (2.3, 1.628520, -21.9),
    (2.4, 1.626290, -22.6),
    (2.5, 1.624000, -23.2),
    (2.6, 1.621660, -23.6),
    (2.7, 1.619280, -24.0),
    (2.8, 1.616870, -24.2),
    (2.9, 1.614450, -24.4),
    (3.0, 1.612000, -24.7),
    (3.1, 1.609510, -25.1),
    (3.2, 1.606970, -25.6),
    (3.3, 1.604380, -26.2),
    (3.4, 1.601730, -26.8),
    (3.5, 1.599020, -27.4),
    (3.6, 1.596260, -27.9),
    (3.7, 1.59344, -28.4),
    (3.8, 1.59057, -29.0),
    (3.9, 1.58764, -29.6),
    (4.0, 1.58465, -30.2),
    (4.2, 1.57848, -31.6),
    (4.4, 1.57202, -32.9),
    (4.6, 1.56533, -34.0),
    (4.8, 1.55845, -34.7),
    (5.0, 1.55145, -35.2),
    (5.2, 1.54436, -35.6),
    (5.4, 1.53721, -35.9),
    (5.6, 1.53000, -36.2),
    (5.8, 1.52273, -36.5),
    (6.0, 1.51541, -36.7),
    (6.5, 1.49698, -36.9),
    (7.0, 1.47868, -36.2),
    (7.5, 1.46086, -35.0),
    (8.0, 1.44374, -33.4),
    (8.5, 1.42747, -31.7),
    (9.0, 1.41207, -29.9),
    (9.5, 1.39751, -28.3),
    (10.0, 1.38373, -26.8),
    (10.5, 1.37065, -25.5),
    (11.0, 1.35820, -24.3),
    (11.5, 1.34632, -23.2),
    (12.0, 1.33499, -22.1),
    (12.5, 1.32416, -21.2),
    (13.0, 1.31381, -20.3),
    (13.5, 1.30390, -19.4),
    (14.0, 1.29439, -18.6),
    (14.5, 1.28526, -17.9),
    (15.0, 1.27645, -17.3),
    (15.5, 1.26794, -16.8),
    (16.0, 1.25967, -16.3),
    (16.5, 1.25161, -15.9),
    (17.0, 1.24372, -15.6),
    (17.5, 1.23596, -15.4),
    (18.0, 1.22830, -15.3),
    (18.5, 1.22070, -15.2),
    (19.0, 1.21311, -15.2),
    (19.5, 1.20548, -15.3),
    (20.0, 1.197748, -15.6),
    (21.0, 1.181548, -17.0),
    (22.0, 1.162797, -21.1),
    (23.0, 1.140817, -20.8),
    (24.0, 1.125923, -9.42),
    (25.0, 1.119448, -4.60),
    (26.0, 1.115658, -3.19),
    (27.0, 1.112810, -2.58),
    (28.0, 1.110421, -2.25),
    (29.0, 1.108261, -2.08),
    (30.0, 1.106244, -1.96),
    (31.0, 1.104324, -1.88),
    (32.0, 1.102476, -1.82),
    (33.0, 1.100681, -1.77),
    (34.0, 1.098930, -1.73),
    (35.0, 1.097216, -1.70),
    (36.0, 1.095534, -1.69),
    (37.0, 1.093878, -1.64),
    (38.0, 1.092244, -1.62),
    (39.0, 1.090627, -1.61),
    (40.0, 1.089024, -1.60),
    (42.0, 1.085842, -1.59),
    (44.0, 1.082669, -1.59),
    (46.0, 1.079492, -1.59),
    (48.0, 1.076303, -1.60),
    (50.0, 1.073099, -1.61),
    (52.0, 1.069881, -1.61),
    (54.0, 1.066650, -1.62),
    (56.0, 1.063403, -1.63),
    (58.0, 1.060141, -1.64),
    (60.0, 1.056862, -1.64),
    (65.0, 1.048584, -1.67),
    (70.0, 1.040183, -1.69),
    (75.0, 1.031651, -1.72),
    (77.35, 1.027594, -1.73),
    (80.0, 1.022984, -1.75),
    (85.0, 1.014181, -1.77),
    (90.0, 1.005244, -1.80),
    (100.0, 0.986974, -1.85),
    (110.0, 0.968209, -1.90),
    (120.0, 0.949000, -1.94),
    (130.0, 0.929390, -1.98),
    (140.0, 0.909416, -2.01),
    (150.0, 0.889114, -2.05),
    (160.0, 0.868518, -2.07),
    (170.0, 0.847659, -2.10),
    (180.0, 0.826560, -2.12),
    (190.0, 0.805242, -2.14),
    (200.0, 0.783720, -2.16),
    (210.0, 0.762007, -2.18),
    (220.0, 0.740115, -2.20),
    (230.0, 0.718054, -2.21),
    (240.0, 0.695834, -2.23),
    (250.0, 0.673462, -2.24),
    (260.0, 0.650949, -2.26),
    (270.0, 0.628302, -2.27),
    (273.0, 0.621141, -2.28),
    (280.0, 0.605528, -2.28),
    (290.0, 0.582637, -2.29),
    (300.0, 0.559639, -2.30),
    (310.0, 0.536542, -2.31),
    (320.0, 0.513361, -2.32),
    (330.0, 0.490106, -2.33),
    (340.0, 0.466760, -2.34),
    (350.0, 0.443371, -2.34),
    (360.0, 0.419960, -2.34),
    (370.0, 0.396503, -2.35),
    (380.0, 0.373002, -2.35),
    (390.0, 0.349453, -2.36),
    (400.0, 0.325839, -2.36),
    (410.0, 0.302161, -2.37),
    (420.0, 0.278416, -2.38),
    (430.0, 0.254592, -2.39),
    (440.0, 0.230697, -2.39),
    (450.0, 0.206758, -2.39),
    (460.0, 0.182832, -2.39),
    (470.0, 0.159010, -2.37),
    (480.0, 0.135480, -2.33),
    (490.0, 0.112553, -2.25),
    (500.0, 0.090681, -2.12),
];
