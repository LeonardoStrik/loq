use crate::expr::{EvalEnv, Expr};
use crate::lexer::Parser;
use std::io::{self, Stdout};
use std::io::{Stdin, Write};
use std::vec::Vec;
pub struct Repl {
    input: String,
    history: Vec<String>,
    stdout: Stdout,
    stdin: Stdin,
    debug_mode: bool,

    quit: bool,
    eval_env: EvalEnv,
}
impl Repl {
    pub fn new() -> Self {
        Repl {
            input: String::new(),
            history: Vec::new(),
            stdin: io::stdin(),
            stdout: io::stdout(),
            debug_mode: true,
            quit: false,
            eval_env: EvalEnv::new(),
        }
    }
    pub fn read_input(&mut self) -> io::Result<()> {
        self.input.clear();
        print!(">  ");
        self.stdout.flush()?;
        self.stdin.read_line(&mut self.input)?;
        self.input.truncate(self.input.len() - 2);
        self.history.push(self.input.clone());
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.quit {
            self.read_input()?;
            match self.input.as_str() {
                "quit;" | "q;" => break,
                "debug;" => {
                    self.debug_mode = !self.debug_mode;
                    println!("Debug mode set to {}", self.debug_mode);
                }
                input => {
                    let mut parser = Parser::from_string(input.to_string());
                    let mut prefix;
                    if let Some(expr) = parser.parse(&self.eval_env) {
                        let val = expr.eval(&mut self.eval_env);
                        prefix = match val {
                            Expr::Numeric(_) => "Num",
                            Expr::Bool(__) => "Bool",
                            _ => "Sym",
                        };
                        if self.debug_mode {
                            println!("  => {prefix}: {val:?}");
                        } else {
                            println!("  => {prefix}: {val}");
                        }
                    };
                }
            }
        }
        Ok(())
    }
}
