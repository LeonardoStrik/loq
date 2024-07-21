use core::fmt;

use crate::{
    expr::Expr,
    lexer::{Loc, Token, TokenKind},
};

pub enum LogLevel {
    Info,
    Warning,
    Error,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
        };
        write!(f, "{}", out)
    }
}
pub struct Diagnoster {}

impl Diagnoster {
    pub fn report(&self, error: ParserError) {
        eprintln!("{}:  {}", LogLevel::Error, &error.to_string());
        self.report_problem_area(error);
    }
    fn report_problem_area(&self, error: ParserError) {
        let problem_loc = match error {
            ParserError::UnexpectedChar { char: _, loc } => loc,
            ParserError::ExpectedToken {
                expected: _,
                found: _,
                while_doing: _,
                loc,
            } => loc,
            ParserError::UnexpectedToken {
                found,
                while_doing: _,
            } => found.loc,
            ParserError::InvalidExpr {
                loc,
                found: _,
                reason: _,
            } => loc,
            ParserError::UnusedParams {
                functor: _,
                func_def: _,
                unused_params: _,
            } => {
                eprintln!("TODO: report UnusedParams");
                return;
            }
            ParserError::InvalidFunParam {
                found: _,
                while_doing: _,
            } => {
                eprintln!("TODO: report invalid function arguments");
                return;
            }
        };
        match problem_loc {
            Loc::Repl { line, idx } => {
                let idx = idx as usize;
                let mut buffer = String::from(" ".repeat(line.len()));
                buffer.insert(idx, '^');
                eprintln!("    {}", line);
                eprintln!("    {}", buffer)
            }
            Loc::File { ln: _, col: _ } => todo!(),
        }
    }
}

pub enum ParserError {
    UnexpectedChar {
        char: char,
        loc: Loc,
    },
    ExpectedToken {
        expected: Vec<TokenKind>,
        found: Option<Token>,
        while_doing: String,
        loc: Loc,
    },
    UnexpectedToken {
        found: Token,
        while_doing: String,
    },
    InvalidExpr {
        loc: Loc,
        found: Box<Expr>,
        reason: String,
    },
    UnusedParams {
        functor: Box<Expr>,
        func_def: Box<Expr>,
        unused_params: Vec<String>,
    },
    InvalidFunParam {
        found: Box<Expr>,
        while_doing: String,
    },
}
fn pretty_enumerate<T: std::fmt::Display>(items: &Vec<T>) -> String {
    let mut out_msg = String::new();
    for (i, kind) in items.iter().enumerate() {
        out_msg.push_str(&kind.to_string());
        if items.len() > 1 {
            if i < items.len() - 2 {
                out_msg.push_str(", ");
            } else if i == items.len() - 2 {
                out_msg.push_str(" or ")
            }
        }
    }
    out_msg
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            ParserError::UnexpectedChar { char, loc: _ } => {
                &format!("Found unexpected '{}'.", char)
            }
            ParserError::ExpectedToken {
                expected,
                found,
                while_doing,
                loc: _,
            } => {
                let mut expected_msg = String::new();
                if expected.len() > 1 {
                    expected_msg.push_str("either ")
                }
                expected_msg.push_str(&pretty_enumerate(expected));
                let found_msg = match found {
                    Some(token) => &token.to_string(),
                    None => "nothing",
                };
                &format!(
                    "Expected {} {}, found {} instead.",
                    expected_msg, while_doing, found_msg
                )
            }
            ParserError::UnexpectedToken { found, while_doing } => {
                &format!("Found unexpected {} {}.", found, while_doing)
            }
            ParserError::InvalidExpr {
                loc: _,
                found,
                reason,
            } => &format!(
                "Found {}. This is an invalid expression because {}",
                found, reason
            ),
            ParserError::UnusedParams {
                functor,
                unused_params,
                func_def,
            } => &format!(
                "Parsed function {} with body: {}, params {} were not used",
                functor,
                func_def,
                pretty_enumerate(unused_params)
            ),
            ParserError::InvalidFunParam { found, while_doing } => &format!(
                "Found {}, which is not a valid parameter, while {}.",
                found, while_doing
            ),
        };
        write!(f, "{}", out)
    }
}
