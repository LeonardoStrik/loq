use std::{collections::HashMap, fmt, iter::zip};

use crate::{diag::Diagnoster, lexer::TokenKind};

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
    pub fn get_precedence(self) -> i32 {
        match self {
            OperatorKind::Pow => 0,
            OperatorKind::Mult => 1,
            OperatorKind::Div => 1,
            OperatorKind::Plus => 2,
            OperatorKind::Min => 2,
            OperatorKind::Equals => 3,
        }
    }
    pub fn from_token_kind(kind: &TokenKind) -> Self {
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

pub struct EvalEnv {
    pub vars: HashMap<String, Box<Expr>>,
    pub funs: HashMap<String, Box<Expr>>,
    pub diag: Diagnoster,
}
impl EvalEnv {
    pub fn new() -> Self {
        EvalEnv {
            vars: HashMap::new(),
            funs: HashMap::new(),
            diag: Diagnoster {},
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    BinOp {
        op_kind: OperatorKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fun {
        name: String,
        params: Vec<Expr>,
    },
    Numeric(f64),
    Variable(String),
    Group(Box<Expr>),
}
impl Expr {
    pub fn eval(&self, eval_env: &mut EvalEnv) -> Expr {
        // top-level entrypoint for evaluation, can insert variable declarations etc
        // this calls eval_recursive for further (non-mutable eval_env) evaluation
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                if *op_kind == OperatorKind::Equals {
                    let mut right = right.clone();
                    match *left.clone() {
                        Expr::Fun { name, params: _ } => {
                            eval_env.funs.insert(name, Box::new(self.clone()));
                        }
                        Expr::Variable(name) => {
                            right = Box::new(right.eval_recursive(eval_env));
                            eval_env.vars.insert(name, right.clone());
                        }
                        _ => panic!("Invalid expression, should not have been parsed"),
                    };
                    Expr::BinOp {
                        op_kind: *op_kind,
                        left: left.clone(),
                        right: right.clone(),
                    }
                } else {
                    self.eval_recursive(eval_env)
                }
            }
            _ => self.eval_recursive(eval_env),
        }
    }
    fn eval_recursive(&self, eval_env: &EvalEnv) -> Expr {
        // evaluates expressions without evaluating equalities, therefore does not need a mut eval_env
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                let a = left.eval_recursive(eval_env);
                let b = right.eval_recursive(eval_env);
                if a.is_num() && b.is_num() {
                    let a = a.expect_val("expect val on is_num==true");
                    let b = b.expect_val("expect val on is_num==true");
                    return match op_kind {
                        OperatorKind::Mult => Expr::Numeric(a * b),
                        OperatorKind::Div => Expr::Numeric(a / b),
                        OperatorKind::Plus => Expr::Numeric(a + b),
                        OperatorKind::Min => Expr::Numeric(a - b),
                        OperatorKind::Pow => Expr::Numeric(a.powf(b)),
                        OperatorKind::Equals => Expr::BinOp {
                            op_kind: *op_kind,
                            left: Box::new(left.eval_recursive(eval_env)),
                            right: Box::new(right.eval_recursive(eval_env)),
                        },
                    };
                }
                let mut right = right.eval_recursive(eval_env);
                let mut op_kind = op_kind;
                if right.is_num() {
                    if right.expect_val("expected val on is_num==true") < 0.0 {
                        match op_kind {
                            OperatorKind::Plus => {
                                op_kind = &OperatorKind::Min;
                                right = Expr::Numeric(
                                    -right.expect_val("expected val on is_num==true"),
                                );
                            }
                            OperatorKind::Min => {
                                op_kind = &OperatorKind::Plus;
                                right = Expr::Numeric(
                                    -right.expect_val("expected val on is_num==true"),
                                );
                            }
                            _ => (),
                        }
                    }
                }
                Expr::BinOp {
                    op_kind: *op_kind,
                    left: Box::new(left.eval_recursive(eval_env)),
                    right: Box::new(right.eval_recursive(eval_env)),
                }
            }
            Expr::Fun {
                name: eval_name,
                params: eval_args,
            } => {
                if let Some(val) = eval_env.funs.get(eval_name) {
                    match *val.clone() {
                        Expr::BinOp {
                            op_kind: _,
                            left,
                            right,
                        } => {
                            if let Expr::Fun {
                                name: _,
                                params: args,
                            } = *left.clone()
                            {
                                // TODO: Find a more convenient way to save functions and evaluate them
                                if args.len() == eval_args.len() {
                                    let mut temp_eval_env = EvalEnv::new();
                                    for (arg_name, arg_value) in zip(args, eval_args) {
                                        let arg_name = arg_name
                                            .expect_name("function argument not a variable");
                                        temp_eval_env
                                            .vars
                                            .insert(arg_name.clone(), Box::new(arg_value.clone()));
                                    }

                                    let mut right = right.eval_recursive(&temp_eval_env);
                                    if !right.is_num() {
                                        right = right.eval_recursive(eval_env);
                                    }
                                    if right.is_num() {
                                        return right;
                                    } else {
                                        return Expr::BinOp {
                                            op_kind: OperatorKind::Equals,
                                            left: left,
                                            right: Box::new(right.eval_recursive(&temp_eval_env)),
                                        };
                                    };
                                }
                            }
                            todo!("not sure how to handle this");
                        }

                        _ => panic!("didn't find BinOp in stashed function definition"),
                    }
                } else {
                    self.clone()
                }
            }
            Expr::Numeric(_) => self.clone(),
            Expr::Variable(name) => {
                if let Some(val) = eval_env.vars.get(name) {
                    *val.clone()
                } else {
                    self.clone()
                }
            }
            Expr::Group(expr) => {
                let expr = expr.eval_recursive(eval_env);
                if expr.is_num() {
                    expr
                } else {
                    Expr::Group(Box::new(expr))
                }
            }
        }
    }
    pub fn expect_val(&self, msg: &str) -> f64 {
        match self {
            Expr::Numeric(val) => *val,
            _ => panic!("{}", msg),
        }
    }
    pub fn expect_name(&self, msg: &str) -> &String {
        match self {
            Expr::Variable(name) => name,
            _ => panic!("{}", msg),
        }
    }
    pub fn is_num(&self) -> bool {
        match self {
            Expr::Numeric(_) => true,
            _ => false,
        }
    }
    pub fn is_var(&self) -> bool {
        match self {
            Expr::Variable(_) => true,
            _ => false,
        }
    }
    pub fn get_var_names(&self) -> Vec<String> {
        match self {
            Expr::BinOp {
                op_kind: _,
                left,
                right,
            } => [left.get_var_names(), right.get_var_names()].concat(),
            Expr::Fun { name: _, params } => {
                let mut result = vec![];
                for param in params {
                    if let Expr::Variable(name) = param {
                        result.push(name.clone());
                    }
                }
                result
            }
            Expr::Numeric(_) => vec![],
            Expr::Variable(name) => vec![name.clone()],
            Expr::Group(expr) => expr.get_var_names(),
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
            } => write!(f, "{}{}{}", left, op_kind, right),
            Expr::Fun { name, params: args } => {
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
            Expr::Group(expr) => write!(f, "({})", expr),
        }
    }
}
