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
    let mut env = ReplEnv::new();

    while !env.quit {
        env.read_input()?;
        match env.input.as_str() {
            "quit;" | "q;" => break,
            // "clear;" | "c;" => {
            //     buffer.clear();
            //     env.history.clear();
            // }
            // "undo;" | "u;" => {
            //     let _ = env.history.pop();
            //     match env.history.pop() {
            //         Some(x) => buffer.truncate(buffer.len() - x.len()),
            //         None => println!("ERROR: Nothing to undo!"),
            //     };
            // }
            input => {
                let mut parser = Parser::from_string(input.to_string());
                if let Some(expr) = parser.parse() {
                    match expr.eval(&mut env.eval_env) {
                        Expr::Numeric(val) => println!("  =>value:  {}", val),
                        otherwise => println!("  =>symbolic  {}", otherwise),
                    }
                }
            }
        };
    }
    Ok(())
}
