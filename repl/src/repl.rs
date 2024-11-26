use loq::expr::{EvalEnv, Expr};
use loq::lexer::Parser;
use std::io::{self, Stdout};
use std::io::{Stdin, Write};
use std::ops::Deref;
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
    fn read_input(&mut self) -> io::Result<()> {
        self.input.clear();
        print!(">  ");
        self.stdout.flush()?;
        self.stdin.read_line(&mut self.input)?;
        self.input = self.input.to_string().trim_end().into();
        self.history.push(self.input.clone());
        Ok(())
    }
    pub fn print_locals(&self) {
        let no_vars = self.eval_env.vars.is_empty();
        if !no_vars {
            println!("---------------------------------------");
            println!("Variables");
            println!("---------------------------------------");
            for (name, value) in self.eval_env.vars.iter() {
                println!("{}: {}", name, value);
            }
        }
        let no_funcs = self.eval_env.funcs.is_empty();
        if !no_funcs {
            println!("---------------------------------------");
            println!("Functors");
            println!("---------------------------------------");
            for (_, body) in self.eval_env.funcs.iter() {
                println!("{}", body);
            }
        }
        if no_vars && no_funcs {
            println!("No variables or functors in local environment!");
        }
    }
    pub fn run(&mut self) -> io::Result<()> {
        while !self.quit {
            self.read_input()?;
            match self.input.as_str() {
                command if command.ends_with(';') => match command.strip_suffix(';').unwrap() {
                    "quit" | "q" => break,
                    "debug" | "db" => {
                        self.debug_mode = !self.debug_mode;
                        println!("Debug mode set to {}", self.debug_mode);
                    }
                    "locals" | "ls" => self.print_locals(),
                    otherwise => println!("Unknown command {}", otherwise),
                },
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
