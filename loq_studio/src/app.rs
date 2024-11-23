use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use ratatui::prelude::Backend;
use ratatui::style::Modifier;
use ratatui::Terminal;

// use loq::expr::{EvalEnv, Expr};
// use loq::lexer::Parser;
use std::io::{self, stderr, Stdout};
use std::io::{Stdin, Write};
use std::time::Duration;
use std::vec::Vec;

use super::ui::draw_ui;

pub enum AppMode {
    Normal,
    Debug,
}
pub enum AppWindow {
    Editor,
    Env,
    Console,
}
impl AppWindow {
    pub fn set_next(&mut self) -> Self {
        match self {
            AppWindow::Editor => AppWindow::Env,
            AppWindow::Env => AppWindow::Console,
            AppWindow::Console => AppWindow::Editor,
        }
    }
}
pub struct App {
    pub input_buffer: Vec<char>,
    pub contents: Vec<String>,
    history: Vec<String>,
    mode: AppMode,
    quit: bool,
    // eval_env: EvalEnv,
    pub currently_active: AppWindow,
}
impl App {
    pub fn new() -> Self {
        App {
            input_buffer: Vec::new(),
            contents: Vec::new(),
            history: Vec::new(),
            mode: AppMode::Debug,
            quit: false,
            // eval_env: EvalEnv::new(),
            currently_active: AppWindow::Console,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|f| draw_ui(f, self))?;
            while let Ok(true) = event::poll(Duration::ZERO) {
                if let Ok(event) = event::read() {
                    match event {
                        Event::FocusGained => (),
                        Event::FocusLost => (),
                        Event::Key(key_event) => self.handle_key_event(key_event),
                        Event::Mouse(mouse_event) => (),
                        Event::Paste(c) => (),
                        Event::Resize(_, _) => (),
                    }
                }
            }
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == event::KeyEventKind::Release {
            // Skip events that are not KeyEventKind::Press
            return;
        }
        match key_event.code {
            KeyCode::Backspace => _ = self.input_buffer.pop(),
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    self.contents.push(self.input_buffer.iter().collect());
                    self.input_buffer.clear();
                }
            }
            KeyCode::Left => (),
            KeyCode::Right => (),
            KeyCode::Up => (),
            KeyCode::Down => (),
            KeyCode::Home => (),
            KeyCode::End => (),
            KeyCode::PageUp => (),
            KeyCode::PageDown => (),
            KeyCode::Tab => self.currently_active = self.currently_active.set_next(),
            KeyCode::BackTab => (),
            KeyCode::Delete => (),
            KeyCode::Insert => (),
            KeyCode::F(_) => (),
            KeyCode::Char(mut ch) => {
                if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    ch = ch.to_ascii_uppercase()
                }
                self.input_buffer.push(ch);
            }
            KeyCode::Null => (),
            KeyCode::Esc => self.quit = true,
            KeyCode::CapsLock => (),
            KeyCode::ScrollLock => (),
            KeyCode::NumLock => (),
            KeyCode::PrintScreen => (),
            KeyCode::Pause => (),
            KeyCode::Menu => (),
            KeyCode::KeypadBegin => (),
            KeyCode::Media(media_key_code) => (),
            KeyCode::Modifier(modifier_key_code) => (),
        }
    }
    pub fn contents_to_text(&self) -> String {
        self.contents.join("\n")
    }
}
