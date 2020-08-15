use rgb::RGB8;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone)]
pub struct ThemeColors {
    pub black: RGB8,
    pub red: RGB8,
    pub green: RGB8,
    pub yellow: RGB8,
    pub blue: RGB8,
    pub magenta: RGB8,
    pub cyan: RGB8,
    pub white: RGB8,
}

impl Invert for ThemeColors {
    fn invert(&mut self) {
        self.black.invert();
        self.red.invert();
        self.green.invert();
        self.yellow.invert();
        self.blue.invert();
        self.magenta.invert();
        self.cyan.invert();
        self.white.invert();
    }
}

#[derive(Debug)]
pub struct Theme {
    /// Background color.
    pub background: RGB8,

    /// Foreground color.
    pub foreground: RGB8,

    /// Normal colors.
    pub normal: ThemeColors,

    /// Bright colors.
    pub bright: ThemeColors,
}

impl Default for Theme {
    fn default() -> Self {
        // TODO add alacritty theme preset for more neutral default
        ColorTheme::Breeze.into()
    }
}

impl Invert for Theme {
    fn invert(&mut self) {
        self.background.invert();
        self.foreground.invert();
        self.normal.invert();
        self.bright.invert();
    }
}

pub trait Invert {
    fn invert(&mut self);
}

impl Invert for RGB8 {
    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ColorTheme {
    Afterglow,
    Argonaut,
    AyuDark,
    AyuMirage,
    Base16DefaultDark,
    Breeze,
    Dracula,
    Gruvbox,
    Kitty,
    Material,
    MonokaiSoda,
}

// TODO field 'family' should just be called 'name' or something
#[derive(Debug)]
pub struct Font {
    /// Font family
    pub family: Option<String>,
    pub size: FontSize,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: None,
            size: FontSize::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FontSize(pub u16);

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for FontSize {
    fn default() -> Self {
        Self(20)
    }
}

impl From<ColorTheme> for Theme {
    fn from(color_theme: ColorTheme) -> Self {
        match color_theme {
            ColorTheme::Afterglow => Self {
                background: RGB8::new(44, 44, 44),
                foreground: RGB8::new(214, 214, 214),
                normal: ThemeColors {
                    black: RGB8::new(28, 28, 28),
                    red: RGB8::new(188, 86, 83),
                    green: RGB8::new(144, 157, 99),
                    yellow: RGB8::new(235, 193, 122),
                    blue: RGB8::new(126, 170, 199),
                    magenta: RGB8::new(170, 98, 146),
                    cyan: RGB8::new(134, 211, 206),
                    white: RGB8::new(202, 202, 202),
                },
                bright: ThemeColors {
                    black: RGB8::new(99, 99, 99),
                    red: RGB8::new(188, 86, 83),
                    green: RGB8::new(144, 157, 99),
                    yellow: RGB8::new(235, 193, 122),
                    blue: RGB8::new(126, 170, 199),
                    magenta: RGB8::new(170, 98, 146),
                    cyan: RGB8::new(134, 211, 206),
                    white: RGB8::new(247, 247, 247),
                },
            },
            ColorTheme::Argonaut => Self {
                background: RGB8::new(41, 44, 62),
                foreground: RGB8::new(235, 235, 235),
                normal: ThemeColors {
                    black: RGB8::new(13, 13, 13),
                    red: RGB8::new(255, 48, 27),
                    green: RGB8::new(160, 229, 33),
                    yellow: RGB8::new(255, 198, 32),
                    blue: RGB8::new(27, 166, 250),
                    magenta: RGB8::new(135, 99, 184),
                    cyan: RGB8::new(33, 222, 239),
                    white: RGB8::new(235, 235, 235),
                },
                bright: ThemeColors {
                    black: RGB8::new(109, 112, 112),
                    red: RGB8::new(255, 67, 82),
                    green: RGB8::new(184, 228, 102),
                    yellow: RGB8::new(255, 215, 80),
                    blue: RGB8::new(27, 166, 250),
                    magenta: RGB8::new(165, 120, 234),
                    cyan: RGB8::new(115, 251, 241),
                    white: RGB8::new(254, 254, 248),
                },
            },
            ColorTheme::AyuDark => Self {
                background: RGB8::new(10, 14, 20),
                foreground: RGB8::new(179, 177, 173),
                normal: ThemeColors {
                    black: RGB8::new(1, 6, 14),
                    red: RGB8::new(234, 108, 115),
                    green: RGB8::new(145, 179, 98),
                    yellow: RGB8::new(249, 175, 79),
                    blue: RGB8::new(83, 189, 250),
                    magenta: RGB8::new(250, 233, 148),
                    cyan: RGB8::new(144, 225, 198),
                    white: RGB8::new(199, 199, 199),
                },
                bright: ThemeColors {
                    black: RGB8::new(104, 104, 104),
                    red: RGB8::new(240, 113, 120),
                    green: RGB8::new(194, 217, 76),
                    yellow: RGB8::new(255, 180, 84),
                    blue: RGB8::new(89, 194, 255),
                    magenta: RGB8::new(255, 238, 153),
                    cyan: RGB8::new(149, 230, 203),
                    white: RGB8::new(255, 255, 255),
                },
            },
            ColorTheme::Gruvbox => Self {
                background: RGB8::new(40, 40, 40),
                foreground: RGB8::new(235, 219, 178),
                normal: ThemeColors {
                    black: RGB8::new(40, 40, 40),
                    red: RGB8::new(204, 36, 29),
                    green: RGB8::new(152, 151, 26),
                    yellow: RGB8::new(215, 153, 33),
                    blue: RGB8::new(69, 133, 136),
                    magenta: RGB8::new(177, 98, 134),
                    cyan: RGB8::new(104, 157, 106),
                    white: RGB8::new(168, 153, 132),
                },
                bright: ThemeColors {
                    black: RGB8::new(146, 131, 116),
                    red: RGB8::new(251, 73, 52),
                    green: RGB8::new(184, 187, 38),
                    yellow: RGB8::new(250, 189, 47),
                    blue: RGB8::new(131, 165, 152),
                    magenta: RGB8::new(211, 134, 155),
                    cyan: RGB8::new(142, 192, 124),
                    white: RGB8::new(235, 219, 178),
                },
            },
            ColorTheme::Material => Self {
                background: RGB8::new(38, 50, 56),
                foreground: RGB8::new(238, 255, 255),
                normal: ThemeColors {
                    black: RGB8::new(0, 0, 0),
                    red: RGB8::new(229, 57, 53),
                    green: RGB8::new(145, 184, 89),
                    yellow: RGB8::new(255, 182, 44),
                    blue: RGB8::new(97, 130, 184),
                    magenta: RGB8::new(255, 83, 112),
                    cyan: RGB8::new(57, 173, 181),
                    white: RGB8::new(160, 160, 160),
                },
                bright: ThemeColors {
                    black: RGB8::new(78, 78, 78),
                    red: RGB8::new(255, 83, 112),
                    green: RGB8::new(195, 232, 141),
                    yellow: RGB8::new(255, 203, 107),
                    blue: RGB8::new(130, 170, 255),
                    magenta: RGB8::new(240, 113, 120),
                    cyan: RGB8::new(137, 221, 255),
                    white: RGB8::new(255, 255, 255),
                },
            },
            ColorTheme::MonokaiSoda => Self {
                background: RGB8::new(26, 26, 26),
                foreground: RGB8::new(196, 197, 181),
                normal: ThemeColors {
                    black: RGB8::new(26, 26, 26),
                    red: RGB8::new(244, 0, 95),
                    green: RGB8::new(152, 224, 36),
                    yellow: RGB8::new(250, 132, 25),
                    blue: RGB8::new(157, 101, 255),
                    magenta: RGB8::new(244, 0, 95),
                    cyan: RGB8::new(88, 209, 235),
                    white: RGB8::new(196, 197, 181),
                },
                bright: ThemeColors {
                    black: RGB8::new(98, 94, 76),
                    red: RGB8::new(244, 0, 95),
                    green: RGB8::new(152, 224, 36),
                    yellow: RGB8::new(224, 213, 97),
                    blue: RGB8::new(157, 101, 255),
                    magenta: RGB8::new(244, 0, 95),
                    cyan: RGB8::new(88, 209, 235),
                    white: RGB8::new(246, 246, 239),
                },
            },
            ColorTheme::AyuMirage => Self {
                background: RGB8::new(32, 39, 52),
                foreground: RGB8::new(203, 204, 198),
                normal: ThemeColors {
                    black: RGB8::new(25, 30, 42),
                    red: RGB8::new(255, 51, 51),
                    green: RGB8::new(186, 230, 126),
                    yellow: RGB8::new(255, 167, 89),
                    blue: RGB8::new(115, 208, 255),
                    magenta: RGB8::new(255, 213, 128),
                    cyan: RGB8::new(149, 230, 203),
                    white: RGB8::new(199, 199, 199),
                },
                bright: ThemeColors {
                    black: RGB8::new(104, 104, 104),
                    red: RGB8::new(242, 121, 131),
                    green: RGB8::new(166, 204, 112),
                    yellow: RGB8::new(255, 204, 102),
                    blue: RGB8::new(92, 207, 230),
                    magenta: RGB8::new(255, 238, 153),
                    cyan: RGB8::new(149, 230, 203),
                    white: RGB8::new(255, 255, 255),
                },
            },
            ColorTheme::Base16DefaultDark => Self {
                background: RGB8::new(24, 24, 24),
                foreground: RGB8::new(216, 216, 216),
                normal: ThemeColors {
                    black: RGB8::new(24, 24, 24),
                    red: RGB8::new(171, 70, 66),
                    green: RGB8::new(161, 181, 108),
                    yellow: RGB8::new(247, 202, 136),
                    blue: RGB8::new(124, 175, 194),
                    magenta: RGB8::new(186, 139, 175),
                    cyan: RGB8::new(134, 193, 185),
                    white: RGB8::new(216, 216, 216),
                },
                bright: ThemeColors {
                    black: RGB8::new(88, 88, 88),
                    red: RGB8::new(171, 70, 66),
                    green: RGB8::new(161, 181, 108),
                    yellow: RGB8::new(247, 202, 136),
                    blue: RGB8::new(124, 175, 194),
                    magenta: RGB8::new(186, 139, 175),
                    cyan: RGB8::new(134, 193, 185),
                    white: RGB8::new(248, 248, 248),
                },
            },
            ColorTheme::Breeze => Self {
                background: RGB8::new(35, 38, 39),
                foreground: RGB8::new(252, 252, 252),
                normal: ThemeColors {
                    black: RGB8::new(35, 38, 39),
                    red: RGB8::new(237, 21, 21),
                    green: RGB8::new(17, 209, 22),
                    yellow: RGB8::new(246, 116, 0),
                    blue: RGB8::new(29, 153, 243),
                    magenta: RGB8::new(155, 89, 182),
                    cyan: RGB8::new(26, 188, 156),
                    white: RGB8::new(252, 252, 252),
                },
                bright: ThemeColors {
                    black: RGB8::new(127, 140, 141),
                    red: RGB8::new(192, 57, 43),
                    green: RGB8::new(28, 220, 154),
                    yellow: RGB8::new(253, 188, 75),
                    blue: RGB8::new(61, 174, 233),
                    magenta: RGB8::new(142, 68, 173),
                    cyan: RGB8::new(22, 160, 133),
                    white: RGB8::new(255, 255, 255),
                },
            },
            ColorTheme::Dracula => Self {
                background: RGB8::new(40, 42, 54),
                foreground: RGB8::new(248, 248, 242),
                normal: ThemeColors {
                    black: RGB8::new(0, 0, 0),
                    red: RGB8::new(255, 85, 85),
                    green: RGB8::new(80, 250, 123),
                    yellow: RGB8::new(241, 250, 140),
                    blue: RGB8::new(202, 169, 250),
                    magenta: RGB8::new(255, 121, 198),
                    cyan: RGB8::new(139, 233, 253),
                    white: RGB8::new(191, 191, 191),
                },
                bright: ThemeColors {
                    black: RGB8::new(87, 91, 112),
                    red: RGB8::new(255, 110, 103),
                    green: RGB8::new(90, 247, 142),
                    yellow: RGB8::new(244, 249, 157),
                    blue: RGB8::new(202, 169, 250),
                    magenta: RGB8::new(255, 146, 208),
                    cyan: RGB8::new(154, 237, 254),
                    white: RGB8::new(230, 230, 230),
                },
            },
            ColorTheme::Kitty => Self {
                background: RGB8::new(0, 0, 0),
                foreground: RGB8::new(221, 221, 221),
                normal: ThemeColors {
                    black: RGB8::new(0, 0, 0),
                    red: RGB8::new(204, 4, 3),
                    green: RGB8::new(25, 203, 0),
                    yellow: RGB8::new(206, 203, 0),
                    blue: RGB8::new(13, 115, 204),
                    magenta: RGB8::new(203, 30, 209),
                    cyan: RGB8::new(13, 205, 205),
                    white: RGB8::new(221, 221, 221),
                },
                bright: ThemeColors {
                    black: RGB8::new(118, 118, 118),
                    red: RGB8::new(242, 32, 31),
                    green: RGB8::new(35, 253, 0),
                    yellow: RGB8::new(255, 253, 0),
                    blue: RGB8::new(26, 143, 255),
                    magenta: RGB8::new(253, 40, 255),
                    cyan: RGB8::new(20, 255, 255),
                    white: RGB8::new(255, 255, 255),
                },
            },
        }
    }
}

#[derive(Debug)]
pub enum ColorThemeParseError {
    UnknownTheme(String),
}

impl fmt::Display for ColorThemeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnknownTheme(ref t) => write!(f, "Unknown theme: {}", t),
        }
    }
}

impl FromStr for ColorTheme {
    type Err = ColorThemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "afterglow" => Ok(Self::Afterglow),
            "argonaut" => Ok(Self::Argonaut),
            "ayu-dark" => Ok(Self::AyuDark),
            "ayu-mirage" => Ok(Self::AyuMirage),
            "base16" => Ok(Self::Base16DefaultDark),
            "breeze" => Ok(Self::Breeze),
            "dracula" => Ok(Self::Dracula),
            "kitty" => Ok(Self::Kitty),
            "gruvbox" => Ok(Self::Gruvbox),
            "material" => Ok(Self::Material),
            "monokai-soda" => Ok(Self::MonokaiSoda),
            _ => Err(ColorThemeParseError::UnknownTheme(s.to_owned())),
        }
    }
}

impl Font {
    pub fn new<S: Into<String>>(family: Option<S>, size: u16) -> Self {
        Self {
            family: match family {
                Some(f) => Some(f.into()),
                None => None,
            },
            size: FontSize(size),
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
font:{}
    size: {}",
            {
                match &self.family {
                    Some(f) => format!(
                        "
    normal:
        family: {}",
                        f
                    ),
                    None => String::new(),
                }
            },
            self.size
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Window {
    width: u16,
    height: u16,
}

impl Default for Window {
    fn default() -> Self {
        Self::new(80, 25)
    }
}

impl Window {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn new_or_default(width: Option<u16>, height: Option<u16>) -> Self {
        match width {
            Some(w) => match height {
                Some(h) => Self::new(w, h),
                None => Self::default(),
            },
            None => Self::default(),
        }
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
window:
    dimensions:
        columns: {}
        lines: {}",
            self.width, self.height
        )
    }
}

#[derive(Debug)]
pub enum WindowSizeParseError {
    // TODO correct tihs error message, this isn't what necessarily happens
    TooManyXs,
    IntParse(
        // index (0 | 1), which integer failed to parse
        u8,
        // parse error
        ParseIntError,
    ),
}

impl fmt::Display for WindowSizeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyXs => {
                f.write_str("too many dimensions; only one 'x' delimiter is allowed")
            }
            Self::IntParse(ref idx, ref e) => write!(
                f,
                "{} integer failed to parse: {}",
                if *idx == 0 { "first" } else { "second" },
                e
            ),
        }
    }
}

impl FromStr for Window {
    type Err = WindowSizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split_terminator('x').collect();

        if args.len() == 2 {
            match args[0].parse::<u16>() {
                Ok(width) => match args[1].parse::<u16>() {
                    Ok(height) => Ok(Self::new(width, height)),
                    Err(e) => Err(Self::Err::IntParse(1, e)),
                },
                Err(e) => Err(Self::Err::IntParse(0, e)),
            }
        } else {
            Err(Self::Err::TooManyXs)
        }
    }
}
