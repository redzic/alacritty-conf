#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery)]

use rgb::*;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = ColoredHelp)]
struct Args {
    // TODO build abstraction around this with enum parsing, to more clearly
    // handle case-insensitivity, etc.
    /// Theme (Gruvbox, Material, etc.)
    #[structopt()]
    theme: ColorTheme,
}

#[derive(Debug)]
enum ColorTheme {
    Material,
    Gruvbox,
}

#[derive(Debug)]
enum ColorThemeParseError {
    UnknownTheme(String),
}

impl std::fmt::Display for ColorThemeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ColorThemeParseError::UnknownTheme(t) => write!(f, "Unknown theme: {}", t),
        }
    }
}

impl FromStr for ColorTheme {
    type Err = ColorThemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "gruvbox" => Ok(ColorTheme::Gruvbox),
            "material" => Ok(ColorTheme::Material),
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
        match color_theme {
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
            _ => unimplemented!(),
        }
    }
}

// TODO: ask on reddit how to make a generic method that accepts a
// String or &str and appropriately converts it into a Cow

#[derive(Debug)]
struct Font {
    family: String,
    size: u16,
}

impl Font {
    fn new<S: Into<String>>(family: S, size: u16) -> Self {
        Font {
            family: family.into(),
            size,
        }
    }
}

impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "\
# font family + size
{}
# dimensions (width/height)
{}
# color theme
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

fn main() {
    let args = Args::from_args();

    let mut alacritty_yml = File::create({
        let mut path = dirs::config_dir().unwrap();
        path.push("alacritty");
        path.push("alacritty.yml");
        path
    })
    .unwrap();

    write!(
        alacritty_yml,
        "# Generated by alacritty-conf ({})\n\n{}",
        chrono::Utc::now().to_rfc2822(),
        Config::new(
            Font::new("IBM Plex Mono", 21),
            args.theme.into(),
            Window::new(80, 25)
        )
    )
    .unwrap();
}
