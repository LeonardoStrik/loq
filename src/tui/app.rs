use crossterm::event;
use ratatui::prelude::Backend;
use ratatui::Terminal;

use crate::expr::{EvalEnv, Expr};
use crate::lexer::Parser;
use std::io::{self, Stdout};
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
    input: String,
    history: Vec<String>,
    stdout: Stdout,
    stdin: Stdin,
    mode: AppMode,
    quit: bool,
    eval_env: EvalEnv,
    pub currently_active: AppWindow,
}
impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            history: Vec::new(),
            stdin: io::stdin(),
            stdout: io::stdout(),
            mode: AppMode::Debug,
            quit: false,
            eval_env: EvalEnv::new(),
            currently_active: AppWindow::Console,
        }
    }
}
impl App {
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|f| draw_ui(f, self))?;
            while let Ok(true) = event::poll(Duration::ZERO) {}
        }
        Ok(())
    }
}
