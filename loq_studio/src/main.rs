use app::App;
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
use std::time::Duration;
use std::{
    env::consts,
    io::{self, stdout, Write},
};
use std::{thread::sleep, time::Instant};
use ui::draw_ui;
mod app;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}
