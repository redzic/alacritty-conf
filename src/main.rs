#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery)]

use rgb::*;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, StructOpt)]
#[structopt(setting = ColoredHelp)]
struct Args {
    /// Color theme. Use -l/--list-themes to print all themes.
    #[structopt()]
    theme: Option<ColorTheme>,

    // TODO check if it's a valid font before setting it
    /// Font to set
    #[structopt(short, long, default_value = "IBM Plex Mono")]
    font: String,

    /// Font size (integer)
    #[structopt(short = "s", long, default_value = "20")]
    font_size: u16,

    /// Print all available themes
    #[structopt(short, long)]
    list_themes: bool,
}

#[derive(Debug)]
enum ColorTheme {
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

#[derive(Debug)]
enum ColorThemeParseError {
    UnknownTheme(String),
}

impl fmt::Display for ColorThemeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorThemeParseError::UnknownTheme(t) => write!(f, "Unknown theme: {}", t),
        }
    }
}

impl FromStr for ColorTheme {
    type Err = ColorThemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "afterglow" => Ok(ColorTheme::Afterglow),
            "argonaut" => Ok(ColorTheme::Argonaut),
            "ayu-dark" => Ok(ColorTheme::AyuDark),
            "ayu-mirage" => Ok(ColorTheme::AyuMirage),
            "base16" => Ok(ColorTheme::Base16DefaultDark),
            "breeze" => Ok(ColorTheme::Breeze),
            "dracula" => Ok(ColorTheme::Dracula),
            "kitty" => Ok(ColorTheme::Kitty),
            "gruvbox" => Ok(ColorTheme::Gruvbox),
            "material" => Ok(ColorTheme::Material),
            "monokai-soda" => Ok(ColorTheme::MonokaiSoda),
            _ => Err(ColorThemeParseError::UnknownTheme(s.to_owned())),
        }
    }
}

trait ToHex {
    fn to_hex(self) -> String;
}

impl ToHex for RGB<u8> {
    fn to_hex(self) -> String {
        format!("#{}", hex::encode([self.r, self.g, self.b]))
    }
}

#[derive(Debug, Default)]
struct ThemeColors {
    black: RGB8,
    red: RGB8,
    green: RGB8,
    yellow: RGB8,
    blue: RGB8,
    magenta: RGB8,
    cyan: RGB8,
    white: RGB8,
}

#[derive(Debug, Default)]
struct Theme {
    /// Background color
    background: RGB8,

    /// Foreground color
    foreground: RGB8,

    /// Normal colors
    normal: ThemeColors,

    /// Bright colors
    bright: ThemeColors,
}

impl From<ColorTheme> for Theme {
    fn from(color_theme: ColorTheme) -> Self {
        // TODO encode this data into separate YAML files and have
        // a build script generate this code at compile time

        // This probably isn't possible with just a plain build.rs file,
        // so just add some kind of utility python script to do it and
        // generate the rust code, then you could just copy-paste it
        // from there

        match color_theme {
            ColorTheme::Afterglow => Theme {
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
            ColorTheme::Argonaut => Theme {
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
            ColorTheme::AyuDark => Theme {
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
            ColorTheme::Gruvbox => Theme {
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
            ColorTheme::Material => Theme {
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
            ColorTheme::MonokaiSoda => Theme {
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
            ColorTheme::AyuMirage => Theme {
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
            ColorTheme::Base16DefaultDark => Theme {
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
            ColorTheme::Breeze => Theme {
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
            ColorTheme::Dracula => Theme {
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
            ColorTheme::Kitty => Theme {
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
struct Font {
    family: String,
    size: u16,
}

impl Font {
    fn new(family: impl Into<String>, size: u16) -> Self {
        Font {
            family: family.into(),
            size,
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
font:
    normal:
        family: {}
    size: {}",
            self.family, self.size
        )
    }
}

#[derive(Debug, Default)]
struct Window {
    width: u16,
    height: u16,
}

impl Window {
    fn new(width: u16, height: u16) -> Self {
        Window { width, height }
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
struct Config {
    font: Font,
    theme: Theme,
    dimensions: Window,
}

impl Config {
    fn new(font: Font, theme: Theme, dimensions: Window) -> Self {
        Self {
            font,
            theme,
            dimensions,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
# Font family + size
{}
# Dimensions (width/height)
{}
colors:
    primary:
        background: '{}'
        foreground: '{}'
    normal:
        black:   '{}'
        red:     '{}'
        green:   '{}'
        yellow:  '{}'
        blue:    '{}'
        magenta: '{}'
        cyan:    '{}'
        white:   '{}'
    bright:
        black:   '{}'
        red:     '{}'
        green:   '{}'
        yellow:  '{}'
        blue:    '{}'
        magenta: '{}'
        cyan:    '{}'
        white:   '{}'",
            self.font,
            // terminal width/height
            self.dimensions,
            // background/foreground
            self.theme.background.to_hex(),
            self.theme.foreground.to_hex(),
            // normal colors
            self.theme.normal.black.to_hex(),
            self.theme.normal.red.to_hex(),
            self.theme.normal.green.to_hex(),
            self.theme.normal.yellow.to_hex(),
            self.theme.normal.blue.to_hex(),
            self.theme.normal.magenta.to_hex(),
            self.theme.normal.cyan.to_hex(),
            self.theme.normal.white.to_hex(),
            // bright colors
            self.theme.bright.black.to_hex(),
            self.theme.bright.red.to_hex(),
            self.theme.bright.green.to_hex(),
            self.theme.bright.yellow.to_hex(),
            self.theme.bright.blue.to_hex(),
            self.theme.bright.magenta.to_hex(),
            self.theme.bright.cyan.to_hex(),
            self.theme.bright.white.to_hex(),
        )
    }
}

// TODO add functionality to retain font information if only theme is specified

fn main() -> Result<(), std::io::Error> {
    let args = Args::from_args();

    // TODO add "invert" flag to invert the colors of a theme

    // TODO add information at the beginning of files
    // so that it can detect if the config file was originally
    // made by this program and use that to keep settings
    // not specified there

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if args.list_themes {
        writeln!(
            &mut stdout,
            "\
afterglow
argonaut
ayu-dark
ayu-mirage
base16
breeze
dracula
kitty
gruvbox
material
monokai-soda"
        )?;
        return Ok(());
    }

    let theme = match args.theme {
        Some(t) => t,
        None => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
            write!(&mut stdout, "error:")?;
            stdout.reset()?;
            write!(&mut stdout, " You must provide a value for '")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            write!(&mut stdout, "<theme>")?;
            stdout.reset()?;

            write!(&mut stdout, "'\nUse ")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(&mut stdout, "-l")?;
            stdout.reset()?;
            write!(&mut stdout, "/")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(&mut stdout, "--list-themes")?;
            stdout.reset()?;
            writeln!(&mut stdout, " to view the available themes.")?;

            std::process::exit(1);
        }
    };

    let mut alacritty_yml = File::create({
        let mut path = dirs::config_dir().unwrap();
        path.push("alacritty");
        path.push("alacritty.yml");
        path
    })
    .unwrap();

    write!(
        alacritty_yml,
        "# generated by alacritty-conf ({})\n\n{}",
        chrono::Utc::now().to_rfc2822(),
        Config::new(
            Font::new(args.font, args.font_size),
            theme.into(),
            Window::new(80, 25)
        )
    )
    .unwrap();

    Ok(())
}
