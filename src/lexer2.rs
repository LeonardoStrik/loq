// TODO: implement AST parsing
// TODO: commutativity in ast?
use std::fmt;
use std::string::String;

#[derive(Debug, Clone)]
pub struct Loc {
    pub ln: i32,
    pub col: i32,
}
#[derive(Debug, Clone)]
pub enum OperatorKind {
    Mult,
    Div,
    Plus,
    Min,
    Pow,
}
#[derive(Debug, Clone)]

pub enum TokenKind {
    OpenParen,
    CloseParen,
    Operator { op_kind: OperatorKind },
    Ident { name: String },
    NumLit { value: i32 },
}
#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self.kind {
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::Operator { op_kind } => match op_kind {
                OperatorKind::Mult => "*",
                OperatorKind::Div => "/",
                OperatorKind::Plus => "+",
                OperatorKind::Min => "-",
                OperatorKind::Pow => "^",
            },
            TokenKind::Ident { name } => name.as_str(),
            TokenKind::NumLit { value } => &value.to_string(),
        };

        write!(f, "{}", output)
    }
}

pub enum Expr {
    BinOp {
        op_kind: OperatorKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fun {
        functor: Token,
        args: Vec<Expr>,
    },
    Value {
        token: Token,
    },
    Symbol {
        token: Token,
    },
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub counter: usize,
}
impl Lexer {
    fn next(&mut self) -> Option<&Token> {
        if let Some(result) = self.tokens.get(self.counter) {
            self.counter += 1;
            Some(result)
        } else {
            None
        }
    }

    pub fn new(input_string: String) -> Result<Self, ()> {
        if let Ok(tokens) = Self::to_tokens(input_string) {
            Ok(Self { tokens, counter: 0 })
        } else {
            Err(())
        }
    }
    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.counter + offset)
    }

    // pub fn parse(&mut self) -> Result<Expr, ()> {
    //     while let Some(token) = self.next() {
    //         match token.kind {
    //             TokenKind::OpenParen => continue,  //todo!(),
    //             TokenKind::CloseParen => continue, //todo!(),
    //             TokenKind::Symbol { name } => match self.next() {
    //                 Some(next_token) => match next_token.kind {
    //                     TokenKind::OpenParen => todo!(),
    //                     TokenKind::Operator { op_kind } => match self.next() {
    //                         Some(right_token) => {
    //                             return Ok(Expr::BinOp {
    //                                 op_kind,
    //                                 left: Box::new(Expr::Symbol { token }),
    //                                 right: Box::new(match right_token.kind {
    //                                     TokenKind::Symbol { name } => {
    //                                         Expr::Symbol { token: right_token }
    //                                     }
    //                                     TokenKind::NumLit { value } => {
    //                                         Expr::Value { token: right_token }
    //                                     }
    //                                     _ => return Err(()),
    //                                 }),
    //                             })
    //                         }
    //                         None => todo!(),
    //                     },
    //                     _ => todo!(),
    //                 },
    //                 None => (),
    //             },
    //             TokenKind::NumLit { value } => todo!(),
    //             _ => todo!(),
    //         }
    //     }
    //     Ok(Expr::Symbol {
    //         token: Token {
    //             kind: TokenKind::Symbol {
    //                 name: "wrong".to_string(),
    //             },
    //             loc: todo!(),
    //         },
    //     })
    // }
    fn to_tokens(input_string: String) -> Result<Vec<Token>, ()> {
        let mut tokens: Vec<Token> = vec![];
        let mut char_stream = input_string.chars().enumerate().peekable();
        while let Some((i, next_char)) = char_stream.next() {
            let current_loc: Loc = Loc {
                ln: 1,
                col: i as i32,
            };
            let next_token = match next_char {
                '(' => Token {
                    kind: TokenKind::OpenParen,
                    loc: current_loc,
                },
                '+' => Token {
                    kind: TokenKind::Operator {
                        op_kind: OperatorKind::Plus,
                    },
                    loc: current_loc,
                },
                ')' => Token {
                    kind: TokenKind::CloseParen,
                    loc: current_loc,
                },
                x if x.is_alphabetic() => {
                    let mut temp = x.to_string();
                    while let Some((_, next_char)) = char_stream.next_if(|x| x.1.is_alphanumeric())
                    {
                        temp.push(next_char)
                    }

                    Token {
                        kind: TokenKind::Ident { name: temp.clone() },
                        loc: current_loc,
                    }
                }
                x if x.is_numeric() => {
                    let mut temp = x.to_string();
                    while let Some((_, next_char)) = char_stream.next_if(|x| x.1.is_numeric()) {
                        temp.push(next_char)
                    }
                    Token {
                        kind: TokenKind::NumLit {
                            value: temp.parse::<i32>().unwrap(),
                        },
                        loc: current_loc,
                    }
                }
                _ => todo!("error handling"),
            };
            tokens.push(next_token);
        }
        Ok(tokens)
    }
}
