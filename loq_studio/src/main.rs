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
const REFRESH_RATE: f32 = 60.0;
const TICK_TIME: f32 = 1.0 / REFRESH_RATE;

pub fn main() -> io::Result<()> {
    // setup terminal
    let mut term = ratatui::init();
    // create app and run it
    let mut app = App::new();
    let res = app.run(&mut term);

    ratatui::restore();
    res
}
