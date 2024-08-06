//! # [Ratatui] Logo example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::io::{self, stdout};

use crossterm::event::{self, Event};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    layout::{Constraint, Layout},
    widgets::RatatuiLogo,
    Terminal, TerminalOptions, Viewport,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = init_terminal()?;
    let result = run(terminal);
    restore_terminal()?;
    println!();
    result
}

fn run(mut terminal: Terminal<impl Backend>) -> Result<(), color_eyre::eyre::Error> {
    loop {
        terminal.draw(|frame| {
            let [top, bottom] =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());
            frame.render_widget("Powered by", top);
            frame.render_widget(RatatuiLogo, bottom);
        })?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn init_terminal() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };
    Terminal::with_options(CrosstermBackend::new(stdout()), options)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}
