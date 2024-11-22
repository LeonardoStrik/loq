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

pub fn main_tui_loop() -> io::Result<()> {
    // setup terminal
    set_panic_hook();
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(_) = res {
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match key.code {
                KeyCode::Backspace => todo!(),
                KeyCode::Enter => todo!(),
                KeyCode::Left => todo!(),
                KeyCode::Right => todo!(),
                KeyCode::Up => todo!(),
                KeyCode::Down => todo!(),
                KeyCode::Home => todo!(),
                KeyCode::End => todo!(),
                KeyCode::PageUp => todo!(),
                KeyCode::PageDown => todo!(),
                KeyCode::Tab => app.currently_active = app.currently_active.set_next(),
                KeyCode::BackTab => todo!(),
                KeyCode::Delete => todo!(),
                KeyCode::Insert => todo!(),
                KeyCode::F(_) => todo!(),
                KeyCode::Char(_) => todo!(),
                KeyCode::Null => todo!(),
                KeyCode::Esc => break,
                KeyCode::CapsLock => todo!(),
                KeyCode::ScrollLock => todo!(),
                KeyCode::NumLock => todo!(),
                KeyCode::PrintScreen => todo!(),
                KeyCode::Pause => todo!(),
                KeyCode::Menu => todo!(),
                KeyCode::KeypadBegin => todo!(),
                KeyCode::Media(media_key_code) => todo!(),
                KeyCode::Modifier(modifier_key_code) => todo!(),
            }
        }
    }
    Ok(())
}
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
