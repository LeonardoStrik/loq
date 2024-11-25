use app::App;
use console::Console;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    text::Text,
    widgets::Dataset,
    Frame, Terminal,
};
use std::{env, time::Duration};
use std::{
    env::consts,
    io::{self, stdout, Write},
};
use std::{thread::sleep, time::Instant};
mod app;
mod console;
mod ui;

fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}
