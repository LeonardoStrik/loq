use std::{collections::HashMap, fmt, iter::zip};

use crate::{diag::Diagnoster, lexer::TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    Equals,
    DoubleEquals,
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
            OperatorKind::DoubleEquals => 3,
        }
    }
    pub fn from_token_kind(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Equals => Self::Equals,
            TokenKind::DoubleEquals => Self::DoubleEquals,
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
            OperatorKind::Mult => "*",
            OperatorKind::Div => "/",
            OperatorKind::Plus => "+",
            OperatorKind::Min => "-",
            OperatorKind::Pow => "^",
            OperatorKind::Equals => "=",
            OperatorKind::DoubleEquals => "==",
        };
        write!(f, "{}", output)
    }
}

pub struct EvalEnv {
    pub vars: HashMap<String, Box<Expr>>,
    pub funcs: HashMap<String, Box<Expr>>,
    pub diag: Diagnoster,
}
impl EvalEnv {
    pub fn new() -> Self {
        EvalEnv {
            vars: HashMap::new(),
            funcs: HashMap::new(),
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
    Bool(bool),
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
                            eval_env.funcs.insert(name, Box::new(self.clone()));
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
                let left = left.eval_recursive(eval_env);
                let right = right.eval_recursive(eval_env);
                if left.is_num() && right.is_num() {
                    // evaluate pure numerical expressions
                    let a = left.expect_val("expect val on is_num==true");
                    let b = right.expect_val("expect val on is_num==true");
                    return match op_kind {
                        //TODO:  maybe overloading addition etc for Expr to simplify?
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
                        OperatorKind::DoubleEquals => {
                            // TODO: decide what to do for symbolic evaluations?
                            // would like to be able to ascertain that f(a,b)==f(a,b) is true
                            let left = left.eval_recursive(eval_env);
                            let right = right.eval_recursive(eval_env);
                            if left == right {
                                return Expr::Bool(true);
                            } else {
                                return Expr::Bool(false);
                            }
                        }
                    };
                }
                if left.is_bool() && right.is_bool() {
                    // evaluate pure boolean expressions
                    let left = left.expect_bool("expected bool on is_bool=true");
                    let right = right.expect_bool("expected bool on is_bool=true");
                    return match op_kind {
                        OperatorKind::DoubleEquals => Expr::Bool(left == right),
                        OperatorKind::Mult => Expr::Bool(left && right),
                        OperatorKind::Equals => todo!(),
                        OperatorKind::Div => todo!(),
                        OperatorKind::Plus => Expr::Bool(left || right),
                        OperatorKind::Min => todo!(),
                        OperatorKind::Pow => todo!(),
                    };
                }
                let mut right = right;
                let mut op_kind = op_kind;
                if right.is_num() {
                    // simplification step, maybe better to factor out with other simplifications?
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
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Expr::Fun {
                name: eval_name,
                params: eval_args,
            } => {
                if let Some(val) = eval_env.funcs.get(eval_name) {
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
                                            right: Box::new(right),
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
                if expr.is_num() || expr.is_bool() || expr.is_var() {
                    expr
                } else {
                    Expr::Group(Box::new(expr))
                }
            }
            Expr::Bool(_) => self.clone(),
        }
    }
    pub fn expect_val(&self, msg: &str) -> f64 {
        match self {
            Expr::Numeric(val) => *val,
            _ => panic!("Called expect _val on {}, with message: {}", self, msg),
        }
    }
    pub fn expect_name(&self, msg: &str) -> &String {
        match self {
            Expr::Variable(name) => name,
            _ => panic!("{}", msg),
        }
    }
    pub fn expect_bool(&self, msg: &str) -> bool {
        match self {
            Expr::Bool(val) => *val,
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
    pub fn is_bool(&self) -> bool {
        match self {
            Expr::Bool(_) => true,
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
            Expr::Bool(_) => vec![],
        }
    }
    pub fn get_fun_names(&self) -> Vec<String> {
        match self {
            Expr::BinOp {
                op_kind: _,
                left,
                right,
            } => [left.get_fun_names(), right.get_fun_names()].concat(),
            Expr::Fun { name, params: _ } => {
                return vec![name.clone()];
            }
            Expr::Numeric(_) => vec![],
            Expr::Variable(_) => vec![],
            Expr::Group(expr) => expr.get_fun_names(),
            Expr::Bool(_) => vec![],
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
            Expr::Bool(val) => write!(f, "{}", val),
        }
    }
}
