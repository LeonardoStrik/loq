use std::fmt;
use std::mem::discriminant;
use std::string::String;

#[derive(Debug, Clone, Copy, Default)]
pub struct Loc {
    pub ln: i32,
    pub col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum TokenKind {
    // TODO: would like to somehow refactor away the need for double definition of operators
    // `parentheses`
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
    const OPERATORS: &'static [TokenKind] = &[
        TokenKind::Mult,
        TokenKind::Div,
        TokenKind::Plus,
        TokenKind::Min,
        TokenKind::Pow,
        TokenKind::Equals,
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
    fn get_precedence(&self) -> i32 {
        OperatorKind::from_token_kind(self).get_precedence()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]

pub enum OperatorKind {
    Equals,
    Mult,
    Div,
    Plus,
    Min,
    Pow,
}
impl OperatorKind {
    fn get_precedence(self) -> i32 {
        match self {
            OperatorKind::Pow => 0,
            OperatorKind::Mult => 1,
            OperatorKind::Div => 1,
            OperatorKind::Plus => 2,
            OperatorKind::Min => 2,
            OperatorKind::Equals => 3,
        }
    }
    fn from_token_kind(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Equals => Self::Equals,
            TokenKind::Mult => Self::Mult,
            TokenKind::Div => Self::Div,
            TokenKind::Plus => Self::Plus,
            TokenKind::Min => Self::Min,
            TokenKind::Pow => Self::Pow,
            _ => panic!("called OperatorKind::fromt_token_kind on a {:?}", kind),
        }
    }
}
impl fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Mult => "*",
            Self::Div => "/",
            Self::Plus => "+",
            Self::Min => "-",
            Self::Pow => "^",
            Self::Equals => "=",
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
            _ => panic!("called to_value on a {:?}", self.kind),
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self.kind {
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::Comma => ",",
            TokenKind::Mult => "*",
            TokenKind::Div => "/",
            TokenKind::Plus => "+",
            TokenKind::Min => "-",
            TokenKind::Pow => "^",
            TokenKind::Equals => "=",
            TokenKind::Ident => &self.value.as_str(),
            TokenKind::NumLit => &self.value.as_str(),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    // TODO: Implement a way to somehow store parens, useful for: partial evaluation of symbolics, more readable printing
    // TODO: decouple Exprs from tokens to simplify evaluation? feels like a cleaner way to represent an Expr anyway, if you were to store/use them then the old Token info is not relevant anyway
    BinOp {
        op_kind: OperatorKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fun {
        name: String,
        args: Vec<Expr>,
    },
    Numeric(f64),
    Variable(String),
}
impl Expr {
    pub fn eval(&self) -> Expr {
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                if let Expr::Numeric(a) = left.eval() {
                    if let Expr::Numeric(b) = right.eval() {
                        return match op_kind {
                            OperatorKind::Mult => Expr::Numeric(a * b),
                            OperatorKind::Div => Expr::Numeric(a / b),
                            OperatorKind::Plus => Expr::Numeric(a + b),
                            OperatorKind::Min => Expr::Numeric(a - b),
                            OperatorKind::Pow => Expr::Numeric(a.powf(b)),
                            _ => panic!("not all operators handled"),
                        };
                    }
                }
                Expr::BinOp {
                    op_kind: *op_kind,
                    left: Box::new(left.eval()),
                    right: Box::new(right.eval()),
                }
            }
            Expr::Fun { name: _, args: _ } => self.clone(),
            Expr::Numeric(_) => self.clone(),
            Expr::Variable(_) => self.clone(),
        }
    }
    pub fn expect_val(&self, msg: &str) -> f64 {
        match self {
            Expr::Numeric(val) => *val,
            _ => panic!("{}", msg),
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
            } => write!(f, "({}{}{})", left, op_kind, right),
            Expr::Fun { name, args } => {
                let mut args_str = String::new();
                for arg in args {
                    args_str.push_str(&arg.to_string());
                    args_str.push(',');
                }
                if !args.is_empty() {
                    args_str.pop();
                }
                write!(f, "{}({})", name, args_str)
            }
            Expr::Numeric(value) => write!(f, "{}", value),
            Expr::Variable(name) => write!(f, "{}", name),
        }
    }
}

pub struct Lexer {
    pub chars: Vec<char>,
    pub counter: usize,
    pub current_loc: Loc,
    peeked_token: Option<Token>,
    is_empty: bool,
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
            is_empty: false,
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
                '=' => Some(Token {
                    kind: TokenKind::Equals,
                    loc: current_loc,
                    value: "=".to_string(),
                }),
                ',' => Some(Token {
                    kind: TokenKind::Comma,
                    loc: current_loc,
                    value: ",".to_string(),
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
                    let dec_sep = '.';
                    let mut found_dec_sep = false;
                    while let Some(next_char) =
                        self.next_char_if(|x| x.is_numeric() || x == dec_sep)
                    {
                        if next_char == dec_sep {
                            if found_dec_sep {
                                todo!("implement error handling")
                            } else {
                                found_dec_sep = true;
                            }
                        }
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
    fn drop_token(&mut self) {
        let _ = self.next_token();
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
    fn parse_operand(&mut self) -> Option<Expr> {
        if let Some(token) = self
            .lexer
            .expect_token_kinds(&vec![&[TokenKind::OpenParen], TokenKind::OPERANDS].concat())
        {
            match token.kind {
                TokenKind::Ident => Some(Expr::Variable(token.value)),
                TokenKind::NumLit => Some(Expr::Numeric(token.to_value())),
                TokenKind::OpenParen => self.parse(false),
                _ => None,
            }
        } else {
            None
        }
    }
    fn parse_binop(&mut self, left: Expr) -> Option<Expr> {
        // TODO: Somehow refactor this to eliminate hella copy-pasting in checking whether to parse next expression or not
        // TODO: implement some kind of checking whether the complete expression was parsed, expecially due to hanging parens, e.g. "1+2)*3" yields 3
        if let Some(operator) = self.lexer.expect_token_kinds(TokenKind::OPERATORS) {
            if let Some(right) = self.parse_operand() {
                while let Some(token) = self.lexer.peek_token() {
                    match token.kind {
                        TokenKind::CloseParen => match self.stash.pop() {
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
                        },
                        x if x.is_operator() => {
                            if x.get_precedence() < operator.kind.get_precedence() {
                                match self.stash.pop() {
                                    Some(prev_expr) => {
                                        let temp_right = self.parse_binop(prev_expr).unwrap();
                                        self.stash.push(temp_right)
                                    }
                                    None => {
                                        let temp_right = self.parse_binop(right.clone()).unwrap();
                                        self.stash.push(temp_right)
                                    }
                                }
                            } else {
                                {
                                    match self.stash.pop() {
                                        Some(right_expr) => {
                                            return Some(Expr::BinOp {
                                                op_kind: OperatorKind::from_token_kind(
                                                    &operator.kind,
                                                ),
                                                left: Box::new(left),
                                                right: Box::new(right_expr),
                                            });
                                        }
                                        None => {
                                            return Some(Expr::BinOp {
                                                op_kind: OperatorKind::from_token_kind(
                                                    &operator.kind,
                                                ),
                                                left: Box::new(left),
                                                right: Box::new(right),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("panicked on token {}", token);
                            panic!("expected an operator or bracket after an operand")
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
        }
        None
    }
    fn parse_functor(&mut self, name: String) -> Option<Expr> {
        let _ = self.lexer.expect_token_kinds(&[TokenKind::OpenParen])?;
        let mut args = vec![];
        while let Some(_) = self.lexer.peek_token() {
            if let Some(arg) = self.parse(true) {
                args.push(arg);
            } else {
                return Some(Expr::Fun { name, args });
            }
        }
        None
    }
    pub fn parse(&mut self, parsing_args: bool) -> Option<Expr> {
        while let Some(peek_token) = self.lexer.peek_token() {
            match peek_token.kind {
                TokenKind::OpenParen => {
                    if let Some(stashed_expr) = self.stash.pop() {
                        {
                            match stashed_expr {
                                Expr::Numeric(_) => todo!("implement implicit differentiation"),
                                Expr::Variable(name) => {
                                    return self.parse_functor(name);
                                }
                                _ => todo!("error handling"),
                            }
                        }
                    }
                    self.lexer.drop_token();
                    let result = self
                        .parse(false)
                        .expect("expected expression after open paren");
                    self.stash.push(result)
                }
                TokenKind::CloseParen => {
                    if !parsing_args {
                        self.lexer.drop_token();
                    }
                    return self.stash.pop();
                }
                TokenKind::Comma => {
                    if parsing_args {
                        self.lexer.drop_token();
                        return self.stash.pop();
                    } else {
                        return None;
                    }
                }
                TokenKind::Ident | TokenKind::NumLit => {
                    let mut expr = self
                        .parse_operand()
                        .expect("matched Ident or NumLit on peek but failed to parse expr");
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
                _ => panic!("panicked on token {}", peek_token),
            }
        }
        self.stash.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn start_test(name: &str) {
        println!("--------------------------------------------");
        println!("start {} test", name);
        println!("--------------------------------------------");
    }
    fn end_test(name: &str) {
        println!("--------------------------------------------");
        println!("end {} test", name);
        println!("--------------------------------------------");
    }
    #[test]
    fn test_expect_token_kinds() {
        start_test("expect_token_kinds");
        let some_string = String::from("(abc+1234*c)^/-abcd=");
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
        assert_eq!(
            lexer
                .expect_token_kinds(&[TokenKind::OPERANDS, &[TokenKind::OpenParen]].concat())
                .unwrap()
                .kind,
            TokenKind::Ident
        );
        assert_token_kind(&mut lexer, TokenKind::Equals);

        end_test("expect_token_kinds");
    }

    #[test]
    fn test_parser() {
        start_test("parser");
        fn test_parser_on_string(input: &str) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse(false).unwrap();
            println!("{}", expr);
            println!("{:#?}", expr);
        }
        test_parser_on_string("abc+1234");
        test_parser_on_string("1234+abc");
        test_parser_on_string("abc-1234");
        test_parser_on_string("abc*1234");
        test_parser_on_string("abc*1234+4321");
        test_parser_on_string("abc*1234+4321*420/69");
        test_parser_on_string("abc*1234+4321*420/69=321+123+(12*3)");

        end_test("parser");
    }
    #[test]

    fn test_expr_eval() {
        start_test("expr_eval");

        fn test_expr_eval_on_string(input: &str, expected: f64) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse(false).expect("failed to parse expression");
            let val = expr.eval().expect_val("could not evaluate expr");
            println!("{} evaluated to {}", expr, val);

            assert_eq!(
                val, expected,
                "evaluating {} did not yield {}",
                expr, expected
            );
        }

        test_expr_eval_on_string("123456", 123456.0);
        test_expr_eval_on_string("123.456", 123.456);
        test_expr_eval_on_string("123+456", 579.0);
        test_expr_eval_on_string("123-456", -333.0);
        test_expr_eval_on_string("123*456", 56088.0);
        test_expr_eval_on_string("2^3", 8.0);
        test_expr_eval_on_string("123/456", 123.0 / 456.0);
        test_expr_eval_on_string("2+3*3", 11.0);
        test_expr_eval_on_string("3*2^3", 24.0);
        test_expr_eval_on_string("1+3*2^4/8+6-3", 10.0);
        test_expr_eval_on_string("1+3*2^4/8+6-31+3*2^4/8+6-3", -9.0);
        test_expr_eval_on_string("(123+456)*2", 1158.0);
        test_expr_eval_on_string("(1+2)*(3-4)^(2*3)", 3.0);
        test_expr_eval_on_string("((1+2)*(3-4))^(2*3)", 729.0);
        test_expr_eval_on_string("(1+2)*(3-4)^((2*3)^3*2)", 3.0);
        end_test("expr_eval");
    }
    #[test]
    fn test_functor_parsing() {
        start_test("functor parsing");
        fn test_functor_parsing_on_str(input: &str) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse(false).expect("failed to parse expression");
            println!("input: {} Evaluated to: {}", input, expr)
        }
        test_functor_parsing_on_str("f(1,2,3)");
        test_functor_parsing_on_str("g(a,b,c)");
        test_functor_parsing_on_str("f(1,a,3)");
        test_functor_parsing_on_str("f(1,g(2))");
        // test_functor_parsing_on_str("");
        end_test("functor parsing");
    }
}
