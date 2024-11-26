use color_eyre::Result;
use core::fmt;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use loq::expr::{EvalEnv, Expr};
use loq::lexer::Parser;
use ratatui::prelude::Backend;
use ratatui::style::Modifier;
use ratatui::Terminal;
use std::fmt::Formatter;
use std::io::{self, stderr, Stdout};
use std::io::{Stdin, Write};
use std::ops::DerefMut;
use std::time::Duration;
use std::vec::Vec;
use tui_textarea::TextArea;

use crate::console::{Console, Line};

use super::ui::draw_ui;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Debug,
}
impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AppMode::Debug => "debug",
                AppMode::Normal => "normal",
            }
        )
    }
}
pub enum AppWindow {
    Editor,
    Env,
    Console,
}
impl AppWindow {
    pub fn get_next(&self) -> Self {
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
    pub eval_env: EvalEnv,
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
            eval_env: EvalEnv::new(),
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
        } else if (key_event.code == KeyCode::Char('k') || key_event.code == KeyCode::Char('K'))
            && key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
            self.currently_active = self.currently_active.get_next()
        }
        match self.currently_active {
            AppWindow::Editor => _ = self.editor.input(key_event),
            AppWindow::Env => (),
            AppWindow::Console => {
                if let Some(input) = self.console.handle_key_event(key_event) {
                    self.handle_console_input(&input);
                }
            }
        }
    }
    pub fn handle_console_input(&mut self, input: &String) {
        match input.as_str() {
            command if command.starts_with('/') => match command.strip_prefix('/').unwrap() {
                "quit" | "q" => self.quit = true,
                "debug" | "db" => {
                    if self.mode == AppMode::Debug {
                        self.mode = AppMode::Normal;
                    } else {
                        self.mode = AppMode::Debug;
                    }
                    self.console
                        .println(Line::OutputLine(format!("App mode set to {}", self.mode)));
                }
                _ => self
                    .console
                    .println(Line::OutputLine(format!("Unknown command {}", command))),
            },
            otherwise => {
                let mut parser = Parser::from_string(otherwise.to_string());
                if let Some(expr) = parser.parse(&self.eval_env) {
                    let val = expr.eval(&mut self.eval_env);
                    match &val {
                        Expr::Fun { name, params } => {
                            self.console.println(Line::OutputLine(format!(
                                "Function {} is not declared in the current environment.",
                                name
                            )));
                            return;
                        }
                        Expr::Variable(name) => {
                            self.console.println(Line::OutputLine(format!(
                                "Variable {} is not declared in the current environment.",
                                name
                            )));
                            return;
                        }
                        _ => (),
                    };
                    let prefix = match val {
                        Expr::Numeric(_) => "Num",
                        Expr::Bool(__) => "Bool",
                        _ => "Sym",
                    };
                    if self.mode == AppMode::Debug {
                        self.console
                            .println(Line::OutputLine(format!("  => {prefix}: {val:?}")));
                    } else {
                        self.console
                            .println(Line::OutputLine(format!("  => {prefix}: {val}")));
                    }
                };
            }
            _ => (),
        }
    }
    pub fn list_eval_env(&self) -> Vec<String> {
        let mut vars = self
            .eval_env
            .vars
            .iter()
            .map(|(var, val)| format!("{} = {}", *var, *val))
            .collect::<Vec<String>>();
        let mut funcs = self
            .eval_env
            .funcs
            .iter()
            .map(|(_, func)| format!("{}", *func))
            .collect::<Vec<String>>();
        let mut out = if vars.len() == 0 {
            vec![]
        } else {
            vec!["Variables".to_string()]
        };
        out.append(&mut vars);
        out.append(&mut if funcs.len() == 0 {
            vec![]
        } else {
            vec!["Functions".to_string()]
        });
        out.append(&mut funcs);
        out
    }
}
