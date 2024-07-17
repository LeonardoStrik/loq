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
    vars: HashMap<String, Box<Expr>>,
    funs: HashMap<String, Box<Expr>>,
    diag: Diagnoster,
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
    Group(Box<Expr>),
}
impl Expr {
    pub fn eval(&self, env: &mut EvalEnv) -> Expr {
        // top-level entrypoint for evaluation, can insert variable declarations etc
        // this calls eval_recursive for further (non-mutable env) evaluation
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                if *op_kind == OperatorKind::Equals {
                    let right = Box::new(right.eval_recursive(env));
                    let expr = Expr::BinOp {
                        op_kind: *op_kind,
                        left: left.clone(),
                        right: right.clone(),
                    };

                    match *left.clone() {
                        Expr::Fun { name, args: _ } => {
                            env.funs.insert(name, Box::new(expr.clone()));
                        }
                        Expr::Variable(name) => {
                            env.vars.insert(name, right);
                        }
                        _ => panic!("Invalid expression, should not have been parsed"),
                    };
                    expr
                } else {
                    self.eval_recursive(env)
                }
            }
            _ => self.eval_recursive(env),
        }
    }
    fn eval_recursive(&self, env: &EvalEnv) -> Expr {
        // evaluates expressions without evaluating equalities, therefore does not need a mut env
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => {
                let a = left.eval_recursive(env);
                let b = right.eval_recursive(env);
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
                            left: Box::new(left.eval_recursive(env)),
                            right: Box::new(right.eval_recursive(env)),
                        },
                    };
                }

                Expr::BinOp {
                    op_kind: *op_kind,
                    left: Box::new(left.eval_recursive(env)),
                    right: Box::new(right.eval_recursive(env)),
                }
            }
            Expr::Fun {
                name: eval_name,
                args: eval_args,
            } => {
                if let Some(val) = env.funs.get(eval_name) {
                    match *val.clone() {
                        Expr::BinOp {
                            op_kind: _,
                            left,
                            right,
                        } => {
                            if let Expr::Fun { name: _, args } = *left.clone() {
                                //TODO: lot more checking for proper functions, e.g. whether all args are symbolic
                                // TODO: potentially evaluate duplicate expressions in parsing already?
                                //  makes the overhead bigger and introduces a need for an env there too, but I'm not a fan of accepting invalid expressions
                                // TODO: Find a more convenient way to save functions and evaluate them
                                if args.len() == eval_args.len() {
                                    let mut temp_env = EvalEnv::new();
                                    for (arg_name, arg_value) in zip(args, eval_args) {
                                        let arg_name = arg_name
                                            .expect_name("function argument not a variable");
                                        temp_env
                                            .vars
                                            .insert(arg_name.clone(), Box::new(arg_value.clone()));
                                    }

                                    let right = right.eval_recursive(&temp_env);
                                    if right.is_num() {
                                        return right;
                                    } else {
                                        return Expr::BinOp {
                                            op_kind: OperatorKind::Equals,
                                            left: left,
                                            right: Box::new(right.eval_recursive(&temp_env)),
                                        };
                                    };
                                }
                            }
                            return Expr::Numeric(1.0);
                        }

                        _ => panic!("didn't find BinOp in stashed function definition"),
                    }
                } else {
                    self.clone()
                }
            }
            Expr::Numeric(_) => self.clone(),
            Expr::Variable(name) => {
                if let Some(val) = env.vars.get(name) {
                    *val.clone()
                } else {
                    self.clone()
                }
            }
            Expr::Group(expr) => expr.eval_recursive(env),
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
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BinOp {
                op_kind,
                left,
                right,
            } => write!(f, "{}{}{}", left, op_kind, right),
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
            Expr::Group(expr) => write!(f, "({})", expr),
        }
    }
}
