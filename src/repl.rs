use crate::expr::{EvalEnv, Expr};
use crate::lexer::Parser;
use std::io::{self, Stdout};
use std::io::{Stdin, Write};
use std::vec::Vec;
struct ReplEnv {
    input: String,
    history: Vec<String>,
    stdout: Stdout,
    stdin: Stdin,
    quit: bool,
    eval_env: EvalEnv,
}
impl ReplEnv {
    pub fn new() -> Self {
        ReplEnv {
            input: String::new(),
            history: Vec::new(),
            stdin: io::stdin(),
            stdout: io::stdout(),
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
}
pub fn repl() -> io::Result<()> {
    let mut repl_env = ReplEnv::new();

    while !repl_env.quit {
        repl_env.read_input()?;
        match repl_env.input.as_str() {
            "quit;" | "q;" => break,
            // "clear;" | "c;" => {
            //     buffer.clear();
            //     repl_env.history.clear();
            // }
            // "undo;" | "u;" => {
            //     let _ = repl_env.history.pop();
            //     match repl_env.history.pop() {
            //         Some(x) => buffer.truncate(buffer.len() - x.len()),
            //         None => println!("ERROR: Nothing to undo!"),
            //     };
            // }
            input => {
                let mut parser = Parser::from_string(input.to_string());
                if let Some(expr) = parser.parse(&repl_env.eval_env) {
                    match expr.eval(&mut repl_env.eval_env) {
                        Expr::Numeric(val) => println!("  =>value:  {}", val),
                        otherwise => println!("  =>symbolic  {}", otherwise),
                    }
                }
            }
        };
    }
    Ok(())
}
