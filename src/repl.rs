use crate::lexer::{Expr, Parser};
use std::io::{self, Stdout};
use std::io::{Stdin, Write};
use std::vec::Vec;
struct Env {
    input: String,
    history: Vec<String>,
    stdout: Stdout,
    stdin: Stdin,
    quit: bool,
}
impl Env {
    pub fn new() -> Self {
        Env {
            input: String::new(),
            history: Vec::new(),
            stdin: io::stdin(),
            stdout: io::stdout(),
            quit: false,
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
    let mut env = Env::new();

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
                match parser.parse(false) {
                    Some(expr) => match expr.eval() {
                        Expr::Numeric(val) => println!("  =>value:  {}", val),
                        otherwise => println!("  =>symbolic  {}", otherwise),
                    },
                    None => println!("unable to parse"),
                }
            }
        };
    }
    Ok(())
}
