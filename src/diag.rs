use core::fmt;

use crate::lexer::{Loc, Token, TokenKind};

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
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            ParserError::UnexpectedChar { char, loc: _ } => &format!("Found unexpected {}.", char),
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
                for (i, kind) in expected.iter().enumerate() {
                    expected_msg.push_str(&kind.to_string());
                    if expected.len() > 1 {
                        if i < expected.len() - 2 {
                            expected_msg.push_str(", ");
                        } else if i == expected.len() - 2 {
                            expected_msg.push_str(" or ")
                        }
                    }
                }
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
        };
        write!(f, "{}", out)
    }
}
