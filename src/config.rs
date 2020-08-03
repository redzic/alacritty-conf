use crate::theme::{Font, FontSize, Theme, Window};
use rgb::RGB8;
use std::fmt;
use yaml_rust::YamlLoader;

#[derive(Debug, Default)]
pub struct Config {
    font: Font,
    theme: Theme,
    dimensions: Window,
}

#[derive(Debug)]
pub struct PartialConfig {
    font: Option<Font>,
    theme: Option<Theme>,
    dimensions: Option<Window>,
}

impl PartialConfig {
    pub const fn new(font: Option<Font>, theme: Option<Theme>, dimensions: Option<Window>) -> Self {
        Self {
            font,
            theme,
            dimensions,
        }
    }

    pub const fn font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    // TODO clean this code up, specify default size in only one place
    pub fn font_size_or_default(&self) -> u16 {
        match &self.font {
            Some(ref f) => f.size,
            None => FontSize::default(),
        }
        .0
    }
}

trait ToHex {
    fn to_hex(self) -> String;
}

impl ToHex for RGB8 {
    fn to_hex(self) -> String {
        format!("#{}", hex::encode([self.r, self.g, self.b]))
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
{}
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

// TODO return result type

/// Parse current alacritty config into a Config struct that we can manipulate.
/// This function WILL ignore all aspects of the config that are not currently
/// implemented.
pub fn parse(source: &str) -> Option<PartialConfig> {
    let parsed = YamlLoader::load_from_str(source).ok()?;
    let parsed = parsed.get(0)?;

    Some(PartialConfig {
        font: Some(Font::new(
            parsed["font"]["normal"]["family"].as_str(),
            parsed["font"]["size"]
                .as_i64()
                // TODO handle error
                .map(|x| FontSize(x as u16))
                .unwrap_or_default()
                .0,
        )),
        // TODO actually parse the theme
        theme: None,
        dimensions: Some(Window::new_or_default(
            parsed["window"]["dimensions"]["columns"]
                .as_i64()
                .map(|x| x as u16),
            parsed["window"]["dimensions"]["lines"]
                .as_i64()
                // TODO add error handling if it doesn't fit in a u16
                .map(|x| x as u16),
        )),
    })
}

pub fn merge(new_config: PartialConfig, old_config: PartialConfig) -> Config {
    Config {
        // using match instead of .unwrap_or_else() because all unwrap methods
        // take ownership of the variable and thus you can't use captures in
        // subsequent unwrap calls because they have already been moved into
        // the closure
        font: match new_config.font {
            Some(font) => font,
            None => old_config.font.unwrap_or_default(),
        },
        // TODO isn't the logic set up so that it's literally impossible
        // for args.theme to not be present?
        theme: match new_config.theme {
            Some(theme) => theme,
            None => old_config.theme.unwrap_or_default(),
        },
        dimensions: match new_config.dimensions {
            Some(dimensions) => dimensions,
            None => old_config.dimensions.unwrap_or_default(),
        },
    }
}
