use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use ratatui::prelude::Backend;
use ratatui::style::Modifier;
use ratatui::Terminal;
use tui_textarea::TextArea;
// use loq::expr::{EvalEnv, Expr};
// use loq::lexer::Parser;
use std::io::{self, stderr, Stdout};
use std::io::{Stdin, Write};
use std::time::Duration;
use std::vec::Vec;

use crate::ui::Console;

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
pub struct App<'a> {
    pub editor: TextArea<'a>,
    pub console: Console,
    history: Vec<String>,
    mode: AppMode,
    quit: bool,
    // eval_env: EvalEnv,
    pub currently_active: AppWindow,
}
impl App<'_> {
    pub fn new() -> Self {
        App {
            console: Console::new(),
            editor: TextArea::default(),
            history: Vec::new(),
            mode: AppMode::Debug,
            quit: false,
            // eval_env: EvalEnv::new(),
            currently_active: AppWindow::Console,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
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
        if key_event.code == KeyCode::Esc {
            self.quit = true
        }
        self.editor.input(key_event);
    }
}
