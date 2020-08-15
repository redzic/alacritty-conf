#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items)]
#![feature(const_option)]

mod config;
mod event;
mod theme;

use crate::config::{Config, PartialConfig};
use crate::event::{Event, Events};
use crate::theme::{ColorTheme, Font, FontSize, Invert, Theme, Window};
use std::fs::{self, File};
use std::io::{self, Write};
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

#[derive(Debug, StructOpt)]
#[structopt(setting = ColoredHelp)]
struct Args {
    /// Color theme. Use -l/--list-themes to view available themes.
    #[structopt()]
    theme: Option<ColorTheme>,

    // TODO check if it's a valid font before setting it
    /// Font family of terminal to set
    #[structopt(short, long)]
    font: Option<String>,

    /// Font size (integer)
    #[structopt(short = "s", long)]
    font_size: Option<u16>,

    /// Print all available themes
    #[structopt(short, long)]
    list_themes: bool,

    /// Invert colors of the selected theme
    #[structopt(short = "I", long)]
    invert_colors: bool,

    /// Dimensions of window size
    #[structopt(long, short, default_value = "80x25")]
    dimensions: Window,

    /// Launch in TUI mode
    #[structopt(long)]
    tui: bool,
}

// App state
struct App {
    mode: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Args::from_args();

    // termcolor stdout/stderr
    let mut tc_stdout = StandardStream::stdout(ColorChoice::Always);
    let mut tc_stderr = StandardStream::stderr(ColorChoice::Always);

    if args.list_themes {
        writeln!(
            tc_stdout,
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

        tc_stdout.flush()?;

        return Ok(());
    }

    // TODO do not error out if none is specified, just get the default one instead
    // and eventually correctly parse existing themes
    let theme = if let Some(theme_preset) = args.theme {
        let mut theme = Theme::from(theme_preset);

        if args.invert_colors {
            theme.invert();
        }

        theme
    } else {
        tc_stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        write!(tc_stderr, "error:")?;
        tc_stderr.reset()?;
        write!(tc_stderr, " You must provide a value for '")?;
        tc_stderr.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(tc_stderr, "<theme>")?;
        tc_stderr.reset()?;

        write!(tc_stderr, "'\nUse ")?;
        tc_stderr.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(tc_stderr, "-l")?;
        tc_stderr.reset()?;
        write!(tc_stderr, "/")?;
        tc_stderr.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(tc_stderr, "--list-themes")?;
        tc_stderr.reset()?;
        writeln!(tc_stderr, " to view the available themes.")?;

        tc_stderr.flush()?;

        return Ok(());
    };

    let path = {
        let mut path = dirs::config_dir().unwrap();
        path.push("alacritty");
        path.push("alacritty.yml");
        path
    };

    // TODO: correct error handling
    let old_config = fs::read_to_string(&path)?;

    let mut new_config = File::create(&path)?;

    // TODO detect if the file does not already exist, and inform the user that
    // since it's the first time that they're creating the config, changes only
    // apply after restatring alacritty

    // TODO validate font

    write!(
        new_config,
        "# generated by alacritty-conf ({})\n\n{}",
        chrono::Utc::now().to_rfc2822(),
        {
            let old_config = config::parse(old_config.as_str());

            match old_config {
                Some(config) => config::merge(
                    PartialConfig::new(
                        match args.font {
                            Some(f) => Some(Font {
                                family: Some(f),
                                size: FontSize(
                                    args.font_size
                                        .unwrap_or_else(|| config.font_size_or_default()),
                                ),
                            }),
                            None => Some(Font {
                                family: match config.font() {
                                    // TODO make font family generic to avoid this mess
                                    Some(f) => f.family.as_ref().map(|f| f.as_str().to_owned()),
                                    None => None,
                                },
                                size: FontSize(
                                    args.font_size
                                        .unwrap_or_else(|| config.font_size_or_default()),
                                ),
                            }),
                        },
                        // TODO maybe these don't need to be wrapped in Option?
                        Some(theme),
                        Some(args.dimensions),
                    ),
                    config,
                ),
                // TODO isn't this problematic?
                // if there isn't a config, or it couldn't be parsed,
                // it should just be overwritten with CLI args right?
                None => Config::default(),
            }
        }
    )?;

    if args.tui {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut events = Events::new();

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let block = Block::default().title("Color themes").borders(Borders::ALL);
                f.render_widget(block, chunks[0]);
            })?;
        }
    }

    Ok(())
}
