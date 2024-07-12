use std::env::args;
use std::fmt;
use std::string::String;

#[derive(Debug, Clone, Copy, Default)]
pub struct Loc {
    pub ln: i32,
    pub col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum TokenKind {
    OpenParen,
    CloseParen,
    Mult,
    Div,
    Plus,
    Min,
    Pow,
    Ident,
    NumLit,
}

impl TokenKind {
    const OPERATORS: &'static [TokenKind] = &[
        TokenKind::Mult,
        TokenKind::Div,
        TokenKind::Plus,
        TokenKind::Min,
        TokenKind::Pow,
    ];
    const OPERANDS: &'static [TokenKind] = &[TokenKind::Ident, TokenKind::NumLit];
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
}
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    value: String,
    pub loc: Loc,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self.kind {
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",

            TokenKind::Mult => "*",
            TokenKind::Div => "/",
            TokenKind::Plus => "+",
            TokenKind::Min => "-",
            TokenKind::Pow => "^",
            TokenKind::Ident => &self.value.as_str(),
            TokenKind::NumLit => &self.value.as_str(),
        };

        write!(f, "{}", output)
    }
}
#[derive(Debug)]
pub enum Expr {
    BinOp {
        op_kind: TokenKind,
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
    Variable {
        token: Token,
    },
}
impl Expr {
    pub fn eval(&self) -> Option<f64> {
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                let a = left.eval().unwrap();
                let b = right.eval().unwrap();
                match op_kind {
                    TokenKind::Mult => Some(a * b),
                    TokenKind::Div => Some(a / b),
                    TokenKind::Plus => Some(a + b),
                    TokenKind::Min => Some(a - b),
                    TokenKind::Pow => Some(a.powf(b)),
                    _ => None,
                }
            }
            Expr::Fun {
                functor: _,
                args: _,
            } => todo!("evaluate function"),
            Expr::Value { token } => Some(
                token
                    .value
                    .parse::<f64>()
                    .expect("failed parsing NumLit to f64"),
            ),
            Expr::Variable { token: _ } => todo!("evaluate variables"),
        }
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => write!(
                f,
                "{}{}{}",
                left,
                Token {
                    kind: *op_kind,
                    value: String::new(),
                    loc: Loc::default()
                },
                right
            ),
            Expr::Fun { functor, args } => todo!(),
            Expr::Value { token } | Expr::Variable { token } => write!(f, "{}", token.value),
        }
    }
}

pub struct Lexer {
    pub chars: Vec<char>,
    pub counter: usize,
    pub current_loc: Loc,
    peeked_token: Option<Token>,
}
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
impl Lexer {
    pub fn from_string(input: String) -> Self {
        Lexer {
            chars: input.chars().collect(),
            counter: 0,
            current_loc: Loc { ln: 0, col: -1 },
            peeked_token: None,
        }
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
            self.counter += 1;
            self.current_loc.col += 1;
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
        if let Some(token) = self.peeked_token.take() {
            Some(token)
        } else {
            self.token_from_chars()
        }
    }
    pub fn peek_token(&mut self) -> Option<Token> {
        let token = self.next_token();
        self.peeked_token = token;
        self.peeked_token.clone()
    }
    fn token_from_chars(&mut self) -> Option<Token> {
        while let Some(next_char) = self.next_char() {
            let current_loc = self.current_loc.clone();
            return match next_char {
                '(' => Some(Token {
                    kind: TokenKind::OpenParen,
                    loc: current_loc,
                    value: "(".to_string(),
                }),
                ')' => Some(Token {
                    kind: TokenKind::CloseParen,
                    loc: current_loc,
                    value: ")".to_string(),
                }),
                '+' => Some(Token {
                    kind: TokenKind::Plus,
                    loc: current_loc,
                    value: "+".to_string(),
                }),
                '-' => Some(Token {
                    kind: TokenKind::Min,
                    loc: current_loc,
                    value: "-".to_string(),
                }),
                '*' => Some(Token {
                    kind: TokenKind::Mult,
                    loc: current_loc,
                    value: "*".to_string(),
                }),
                '/' => Some(Token {
                    kind: TokenKind::Div,
                    loc: current_loc,
                    value: "/".to_string(),
                }),
                '^' => Some(Token {
                    kind: TokenKind::Pow,
                    loc: current_loc,
                    value: "^".to_string(),
                }),
                x if x.is_alphabetic() => {
                    let mut temp = x.to_string();
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
                    let mut temp = x.to_string();
                    while let Some(next_char) = self.next_char_if(|x| x.is_numeric()) {
                        temp.push(next_char)
                    }
                    Some(Token {
                        kind: TokenKind::NumLit,
                        loc: current_loc,
                        value: temp.clone(),
                    })
                }
                _ => None,
            };
        }
        None
    }
    fn expect_token_kinds(&mut self, expected: &[TokenKind]) -> Option<Token> {
        if let Some(token) = self.peek_token() {
            if token.kind.is_in(expected) {
                return self.next_token();
            }
        }
        None
    }
}

pub struct Parser {
    lexer: Lexer,
    stash: Vec<Expr>,
}
impl Parser {
    pub fn from_string(input: String) -> Self {
        Parser {
            lexer: Lexer::from_string(input),
            stash: vec![],
        }
    }
    fn parse_expr(&mut self) -> Option<Expr> {
        if let Some(token) = self.lexer.expect_token_kinds(TokenKind::OPERANDS) {
            match token.kind {
                TokenKind::Ident => Some(Expr::Variable { token }),
                TokenKind::NumLit => Some(Expr::Value { token }),
                _ => None,
            }
        } else {
            None
        }
    }
    fn parse_binop(&mut self, left: Expr) -> Option<Expr> {
        if let Some(operator) = self.lexer.expect_token_kinds(TokenKind::OPERATORS) {
            if let Some(right) = self.parse_expr() {
                return Some(Expr::BinOp {
                    op_kind: operator.kind,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }
        None
    }
    pub fn parse(&mut self) -> Option<Expr> {
        while let Some(peek_token) = self.lexer.peek_token() {
            match peek_token.kind {
                TokenKind::OpenParen => todo!(),
                TokenKind::Ident | TokenKind::NumLit => {
                    let left = self
                        .parse_expr()
                        .expect("matched Ident on peek but failed to parse expr");
                    let expr = self.parse_binop(left).expect("Expected binary operator");
                    self.stash.push(expr);
                }
                x if x.is_operator() => {
                    let left = self
                        .stash
                        .pop()
                        .expect("Expected an operand before an operator");
                    let expr = self
                        .parse_binop(left)
                        .expect("expected an operand after operator");
                    self.stash.push(expr)
                }
                _ => panic!(),
            }
        }
        self.stash.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let some_string = String::from("abc+1234+c");
        let mut buffer = " ".repeat(some_string.len());
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::from_string(some_string);
        loop {
            if let Some(token) = lexer.peek_token() {
                println!("{}", token);
            }
            match lexer.next_token() {
                Some(token) => tokens.push(token),
                None => break,
            }
        }
        for token in tokens {
            print!("{}", token);
            buffer.insert(token.loc.col as usize, '^');
        }
        println!();
        println!("{}", buffer);
    }
    #[test]
    fn test_expect_token_kinds() {
        let some_string = String::from("(abc+1234*c)^/-");
        let mut lexer = Lexer::from_string(some_string);
        fn assert_token_kind(lexer: &mut Lexer, kind: TokenKind) {
            assert_eq!(
                (&lexer.expect_token_kinds(&[kind.clone()]).unwrap().kind),
                (&kind),
                "Expected{:?} but got {:?}",
                lexer.expect_token_kinds(&[kind.clone()]).unwrap().kind,
                kind
            );
        }
        assert_token_kind(&mut lexer, TokenKind::OpenParen);
        assert_token_kind(&mut lexer, TokenKind::Ident);
        assert_token_kind(&mut lexer, TokenKind::Plus);
        assert_token_kind(&mut lexer, TokenKind::NumLit);
        assert_token_kind(&mut lexer, TokenKind::Mult);
        assert_token_kind(&mut lexer, TokenKind::Ident);
        assert_token_kind(&mut lexer, TokenKind::CloseParen);
        assert_token_kind(&mut lexer, TokenKind::Pow);
        assert_token_kind(&mut lexer, TokenKind::Div);
        assert_token_kind(&mut lexer, TokenKind::Min);
    }

    #[test]
    fn test_parser() {
        fn test_parser_on_string(input: String) {
            let mut parser = Parser::from_string(input);
            let expr = parser.parse().unwrap();
            println!("{}", expr);
            println!("{:#?}", expr);
        }
        let some_string = String::from("abc+1234");
        test_parser_on_string(some_string);
        let some_string = String::from("1234+abc");
        test_parser_on_string(some_string);
        let some_string = String::from("abc-1234");
        test_parser_on_string(some_string);
        let some_string = String::from("abc*1234");
        test_parser_on_string(some_string);
        let some_string = String::from("abc*1234+4321");
        test_parser_on_string(some_string);
        let some_string = String::from("abc*1234+4321*420/69");
        test_parser_on_string(some_string);
    }
    #[test]

    fn test_expr_eval() {
        fn test_expr_eval_on_string(input: String, expected: f64) {
            let mut parser = Parser::from_string(input);
            let expr = parser.parse().expect("failed to parse expression");
            let val = expr.eval().expect("could not evaluate expr");
            assert_eq!(
                val, expected,
                "evaluating {} did not yield {}",
                expr, expected
            );
        }

        let some_string = String::from("123+456");
        test_expr_eval_on_string(some_string, 579.0);
        let some_string = String::from("123-456");
        test_expr_eval_on_string(some_string, -333.0);
        let some_string = String::from("123*456");
        test_expr_eval_on_string(some_string, 56088.0);
        let some_string = String::from("2^3");
        test_expr_eval_on_string(some_string, 8.0);
        let some_string = String::from("123/456");
        test_expr_eval_on_string(some_string, 123.0 / 456.0);
    }
}
