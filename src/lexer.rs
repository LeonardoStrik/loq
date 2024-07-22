use std::string::String;
use std::{fmt, fs};

use crate::diag::ParserError;
use crate::expr::EvalEnv;
use crate::{
    diag::Diagnoster,
    expr::{Expr, OperatorKind},
};

#[derive(Debug, Clone)]
pub enum Loc {
    Repl { line: String, idx: i32 },
    File { ln: i32, col: i32 },
}
impl Loc {
    fn increment(&mut self) {
        *self = match self {
            Loc::Repl { line, idx } => Loc::Repl {
                line: line.to_string(),
                idx: *idx + 1,
            },
            Loc::File { ln: _, col: _ } => todo!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]

pub enum TokenKind {
    // TODO: would like to somehow refactor away the need for double definition of operators
    // EOL
    EOL,
    // parentheses
    OpenParen,
    CloseParen,
    //separators
    Comma,
    // operators
    Equals,
    Mult,
    Div,
    Plus,
    Min,
    Pow,
    // operands
    Ident,
    NumLit,
}

impl TokenKind {
    pub const OPERATORS: &'static [TokenKind] = &[
        TokenKind::Mult,
        TokenKind::Div,
        TokenKind::Plus,
        TokenKind::Min,
        TokenKind::Pow,
        TokenKind::Equals,
    ];
    pub const OPERANDS: &'static [TokenKind] = &[TokenKind::Ident, TokenKind::NumLit];
    fn is_in(self, expected: &[TokenKind]) -> bool {
        for kind in expected {
            if (*kind) == (self) {
                return true;
            }
        }
        false
    }
    fn is_operator(self) -> bool {
        self.is_in(TokenKind::OPERATORS)
    }
    fn is_operand(self) -> bool {
        self.is_in(TokenKind::OPERANDS)
    }
    fn get_precedence(&self) -> i32 {
        OperatorKind::from_token_kind(self).get_precedence()
    }
}
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::Comma => ",",
            TokenKind::Mult => "*",
            TokenKind::Div => "/",
            TokenKind::Plus => "+",
            TokenKind::Min => "-",
            TokenKind::Pow => "^",
            TokenKind::Equals => "=",
            TokenKind::Ident => "Ident",
            TokenKind::NumLit => "NumLit",
            TokenKind::EOL => "\\n",
        };

        write!(f, "{}", output)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    value: String,
    pub loc: Loc,
}
impl Token {
    fn to_value(self) -> f64 {
        match self.kind {
            TokenKind::NumLit => self
                .value
                .parse::<f64>()
                .expect("failed to parse NumLit in to_value"),
            _ => panic!("called to_value on a {}", self),
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self.kind {
            TokenKind::Ident => self.value.as_str(),
            TokenKind::NumLit => self.value.as_str(),
            otherwise => &otherwise.to_string(),
        };

        write!(f, "{}", output)
    }
}

pub struct Lexer {
    pub chars: Vec<char>,
    pub counter: usize,
    pub current_loc: Loc,
    peeked_token: Option<Token>,
    diag: Diagnoster,
}

impl Lexer {
    pub fn from_string(input: String) -> Self {
        Lexer {
            chars: input.chars().collect(),
            counter: 0,
            current_loc: Loc::Repl {
                line: input.chars().collect(),
                idx: -1,
            },
            peeked_token: None,
            diag: Diagnoster {},
        }
    }
    pub fn is_empty(&mut self) -> bool {
        if let Some(_) = self.peek_char() {
            false
        } else {
            true
        }
    }
    fn increment(&mut self) {
        self.counter += 1;
        self.current_loc.increment()
    }
    fn peek_char(&mut self) -> Option<char> {
        if let Some(result) = self.chars.get(self.counter) {
            Some(*result)
        } else {
            None
        }
    }
    fn next_char(&mut self) -> Option<char> {
        if let Some(result) = self.peek_char() {
            self.increment();
            Some(result)
        } else {
            None
        }
    }
    fn next_char_if(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(next_char) = self.peek_char() {
            if predicate(next_char) {
                return self.next_char();
            }
        }
        None
    }
    pub fn next_token(&mut self) -> Option<Token> {
        //TODO: fix is_empty management, causes issues with malformed expressions
        // maybe have a flag for malformed ?
        if let Some(token) = self.peeked_token.take() {
            Some(token)
        } else {
            if let Some(token) = self.token_from_chars() {
                Some(token)
            } else {
                None
            }
        }
    }
    pub fn peek_token(&mut self) -> Option<Token> {
        let token = self.next_token();
        self.peeked_token = token;
        self.peeked_token.clone()
    }
    fn token_from_chars(&mut self) -> Option<Token> {
        while let Some(peek_char) = self.peek_char() {
            if peek_char == ' ' {
                let _ = self.next_char();
                continue;
            }
            let current_loc = self.current_loc.clone();
            return match peek_char {
                '\n' => Some(Token {
                    kind: TokenKind::EOL,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '(' => Some(Token {
                    kind: TokenKind::OpenParen,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                ')' => Some(Token {
                    kind: TokenKind::CloseParen,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '=' => Some(Token {
                    kind: TokenKind::Equals,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                ',' => Some(Token {
                    kind: TokenKind::Comma,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '+' => Some(Token {
                    kind: TokenKind::Plus,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '-' => Some(Token {
                    kind: TokenKind::Min,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '*' => Some(Token {
                    kind: TokenKind::Mult,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '/' => Some(Token {
                    kind: TokenKind::Div,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                '^' => Some(Token {
                    kind: TokenKind::Pow,
                    loc: current_loc,
                    value: self.next_char().unwrap().to_string(),
                }),
                x if x.is_alphabetic() => {
                    let mut temp = self.next_char().unwrap().to_string();
                    while let Some(next_char) = self.next_char_if(|x| x.is_alphanumeric()) {
                        temp.push(next_char)
                    }

                    Some(Token {
                        kind: TokenKind::Ident,
                        loc: current_loc,
                        value: temp.clone(),
                    })
                }
                x if x.is_numeric() => {
                    let mut temp = self.next_char().unwrap().to_string();
                    let dec_sep = '.';
                    let mut found_dec_sep = false;
                    while let Some(next_char) =
                        self.next_char_if(|x| x.is_numeric() || x == dec_sep)
                    {
                        if next_char == dec_sep {
                            if found_dec_sep {
                                self.diag.report(ParserError::UnexpectedChar {
                                    char: next_char,
                                    loc: self.current_loc.clone(),
                                });
                                return None;
                            } else {
                                found_dec_sep = true;
                            }
                        }
                        temp.push(next_char)
                    }
                    if let Some(peeked_char) = self.peek_char() {
                        if peeked_char.is_alphabetic() {
                            self.diag.report(ParserError::UnexpectedChar {
                                char: peek_char,
                                loc: current_loc,
                            });
                            return None;
                        }
                    }
                    Some(Token {
                        kind: TokenKind::NumLit,
                        loc: current_loc,
                        value: temp.clone(),
                    })
                }
                otherwise => {
                    self.diag.report(ParserError::UnexpectedChar {
                        char: otherwise,
                        loc: current_loc,
                    });
                    None
                }
            };
        }
        None
    }
    pub fn expect_token_kinds(
        &mut self,
        expected: &[TokenKind],
        while_doing: String,
    ) -> Option<Token> {
        if let Some(token) = self.peek_token() {
            if token.kind.is_in(expected) {
                return self.next_token();
            }
        }
        let found = self.peek_token();
        self.diag.report(ParserError::ExpectedToken {
            expected: expected.to_vec(),
            found,
            while_doing,
            loc: self.current_loc.clone(),
        });
        None
    }
    fn drop_token(&mut self) {
        let _ = self.next_token();
    }
}

pub struct Parser {
    lexer: Lexer,
    stash: Vec<Expr>,
    diag: Diagnoster,
    depth: i32,
}
impl Parser {
    pub fn from_string(input: String) -> Self {
        Parser {
            lexer: Lexer::from_string(input),
            stash: vec![],
            diag: Diagnoster {},
            depth: 0,
        }
    }
    pub fn from_file(input_path: &str) -> Option<Self> {
        let input = fs::read_to_string(input_path).ok()?;
        let input = input.replace("\r", "");
        Some(Parser {
            lexer: Lexer::from_string(input),
            stash: vec![],
            diag: Diagnoster {},
            depth: 0,
        })
    }

    fn parse_operand(&mut self, eval_env: &EvalEnv) -> Option<Expr> {
        let token = self.lexer.expect_token_kinds(
            &[&[TokenKind::OpenParen], TokenKind::OPERANDS].concat(),
            "while parsing operand".to_string(),
        )?;
        match token.kind {
            TokenKind::Ident => {
                if let Some(next_token) = self.lexer.peek_token() {
                    if next_token.kind == TokenKind::OpenParen {
                        return self.parse_functor(token.value, eval_env);
                    }
                }
                Some(Expr::Variable(token.value))
            }
            TokenKind::NumLit => Some(Expr::Numeric(token.to_value())),
            TokenKind::OpenParen => {
                let operand = self.parse_impl(eval_env, false)?;
                let _ = self.lexer.expect_token_kinds(
                    &[TokenKind::CloseParen],
                    "while parsing expression between parentheses".to_string(),
                )?;
                Some(Expr::Group(Box::new(operand)))
            }
            _ => None,
        }
    }
    fn parse_binop(&mut self, left: Expr, eval_env: &EvalEnv) -> Option<Expr> {
        let operator = self.lexer.expect_token_kinds(
            TokenKind::OPERATORS,
            "while parsing binary operator".to_string(),
        )?;
        if operator.kind == TokenKind::Equals {
            match left {
                Expr::Variable(_) => (),
                Expr::Fun { name: _, params: _ } => (),
                _ => {
                    self.diag.report(ParserError::InvalidExpr {
                        loc: operator.loc,
                        found: Box::new(left),
                        reason: "can only assign values to a variable".to_string(),
                    });
                    return None;
                }
            }
        }
        let right = self.parse_operand(eval_env)?;
        while let Some(token) = self.lexer.peek_token() {
            match token.kind {
                TokenKind::CloseParen => break,
                x if x.is_operator() => {
                    if x.get_precedence() < operator.kind.get_precedence() {
                        match self.stash.pop() {
                            Some(prev_expr) => {
                                let temp_right = self.parse_binop(prev_expr, eval_env)?;
                                self.stash.push(temp_right)
                            }
                            None => {
                                let temp_right = self.parse_binop(right.clone(), eval_env)?;
                                self.stash.push(temp_right)
                            }
                        }
                    } else {
                        break;
                    }
                }
                TokenKind::EOL => break,
                _ => {
                    self.diag.report(ParserError::UnexpectedToken {
                        found: token,
                        while_doing: "while parsing binary operator".to_string(),
                    });
                    return None;
                }
            }
        }
        match self.stash.pop() {
            Some(right_expr) => {
                return Some(Expr::BinOp {
                    op_kind: OperatorKind::from_token_kind(&operator.kind),
                    left: Box::new(left),
                    right: Box::new(right_expr),
                });
            }
            None => {
                return Some(Expr::BinOp {
                    op_kind: OperatorKind::from_token_kind(&operator.kind),
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }
    }
    fn parse_functor(&mut self, name: String, eval_env: &EvalEnv) -> Option<Expr> {
        let _ = self
            .lexer
            .expect_token_kinds(&[TokenKind::OpenParen], "while parsing functor".to_string())?;
        let mut args = vec![];
        while let Some(_) = self.lexer.peek_token() {
            if let Some(arg) = self.parse_impl(eval_env, true) {
                args.push(arg);
                let token = self.lexer.expect_token_kinds(
                    &[TokenKind::CloseParen, TokenKind::Comma],
                    "while parsing functor".to_string(),
                )?;
                match token.kind {
                    TokenKind::CloseParen => {
                        if let Some(func_def) = eval_env.funs.get(&name) {
                            if let Some(token) = self.lexer.peek_token() {
                                if token.kind == TokenKind::Equals {
                                    return Some(Expr::Fun { name, params: args });
                                }
                            }
                            if let Expr::BinOp {
                                op_kind: _,
                                left,
                                right: _,
                            } = *func_def.clone()
                            {
                                if let Expr::Fun { name, params } = *left {
                                    if params.len() != args.len() {
                                        self.diag.report(ParserError::InvalidExpr {
                                            loc: token.loc,
                                            found: Box::new(Expr::Fun { name:name.clone(), params: args }),
                                            reason: format!("function {} is already defined as {}, and the number of arguments doesn't match",name,func_def),
                                        });
                                        return None;
                                    }
                                }
                            }
                        }
                        return Some(Expr::Fun { name, params: args });
                    }
                    TokenKind::Comma => continue,
                    _ => panic!("found not comma or close paren while expecting them"),
                }
            } else {
                return Some(Expr::Fun { name, params: args });
            }
        }
        None
    }
    pub fn parse(&mut self, eval_env: &EvalEnv) -> Option<Expr> {
        assert!(
            self.stash.is_empty(),
            "Expected stash to be empty when starting parsing, must be an implementation error"
        );
        if let Some(result) = self.parse_impl(eval_env, false) {
            if !self.lexer.is_empty() {
                if let None = self.lexer.expect_token_kinds(
                    &[TokenKind::EOL],
                    "while returning from parsing".to_string(),
                ) {
                    return None;
                }
            }
            // if result is a function definition, check whether all parameters are used
            if let Expr::BinOp {
                op_kind,
                left,
                right,
            } = result.clone()
            {
                if op_kind == OperatorKind::Equals {
                    if let Expr::Fun { name, params: args } = *left.clone() {
                        if right.get_fun_names().contains(&name) {
                            self.diag.report(ParserError::RecusiveFuncDef {
                                func_def: Box::new(result),
                            });
                            return None;
                        }
                        let used_vars = right.get_var_names();
                        let mut unused_params = vec![];
                        for param in args {
                            match param {
                                Expr::Variable(name) => {
                                    if !used_vars.contains(&name) {
                                        unused_params.push(name)
                                    }
                                }
                                _ => {
                                    self.diag.report(ParserError::InvalidFuncParam {
                                        found: Box::new(param),
                                        while_doing: format!("parsing {}", result),
                                    });
                                    return None;
                                }
                            }
                        }
                        if unused_params.len() > 0 {
                            self.diag.report(ParserError::UnusedParams {
                                functor: left,
                                func_def: right,
                                unused_params,
                            });
                            return None;
                        }
                    }
                }
            }
            return Some(result);
        }
        None
    }
    fn parse_impl(&mut self, eval_env: &EvalEnv, parsing_args: bool) -> Option<Expr> {
        self.depth += 1;
        while let Some(peek_token) = self.lexer.peek_token() {
            match peek_token.kind {
                TokenKind::EOL => {
                    break;
                }
                TokenKind::OpenParen => {
                    if let Some(stashed_expr) = self.stash.pop() {
                        {
                            println!("matched stashed expr {}", stashed_expr);
                            match stashed_expr {
                                Expr::Variable(name) => {
                                    self.depth -= 1;
                                    return self.parse_functor(name, eval_env);
                                }

                                _ => {
                                    self.diag.report(ParserError::UnexpectedToken {
                                        found: peek_token,
                                        while_doing: format!(
                                            "parsing expression after {}",
                                            stashed_expr
                                        ),
                                    });
                                    return None;
                                }
                            }
                        }
                    }
                    self.lexer.drop_token();
                    let result = self.parse_impl(eval_env, false)?;
                    let _ = self.lexer.expect_token_kinds(
                        &[TokenKind::CloseParen],
                        "while parsing expression between parens".to_string(),
                    )?;
                    self.stash.push(Expr::Group(Box::new(result)))
                }
                TokenKind::CloseParen => {
                    break;
                }
                TokenKind::Comma => {
                    if parsing_args {
                        break;
                    } else {
                        self.diag.report(ParserError::UnexpectedToken {
                            found: peek_token,
                            while_doing: "not parsing args".to_string(),
                        });
                        return None;
                    }
                }
                TokenKind::Ident | TokenKind::NumLit => {
                    if let Some(expr) = self.stash.last() {
                        let while_doing = format!("Parsing after expression {}", expr);
                        self.diag.report(ParserError::UnexpectedToken {
                            found: peek_token,
                            while_doing,
                        });
                        return None;
                    }
                    let expr = self.parse_operand(eval_env)?;
                    self.stash.push(expr);
                }
                x if x.is_operator() => {
                    if x == TokenKind::Equals && self.depth != 1 {
                        let while_doing = "while not parsing a top-level operator.\
                         Equals is only allowed as the main expression, not in a subexpression"
                            .to_string();

                        self.diag.report(ParserError::UnexpectedToken {
                            found: peek_token,
                            while_doing,
                        });
                        return None;
                    }
                    let left = self.stash.pop()?;
                    let expr = self.parse_binop(left, eval_env)?;
                    self.stash.push(expr)
                }
                TokenKind::Equals
                | TokenKind::Mult
                | TokenKind::Div
                | TokenKind::Plus
                | TokenKind::Min
                | TokenKind::Pow => {
                    let msg = match parsing_args {
                        true => "parsing function arguments",
                        false => "parsing",
                    }
                    .to_string();
                    self.diag.report(ParserError::UnexpectedToken {
                        found: peek_token,
                        while_doing: msg,
                    });
                    return None;
                }
            }
        }
        self.depth -= 1;
        self.stash.pop()
    }
}
