#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        expr::Expr,
        lexer::{Lexer, Parser, TokenKind},
    };

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
        let some_string = String::from(" ( abc + 1234 * c ) ^ / - abcd = ");
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
            let mut env = HashMap::new();
            let val = expr.eval(&mut env).expect_val("could not evaluate expr");
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

    #[test]
    fn test_var_eval() {
        let mut env = HashMap::new();

        fn test_var_eval_on_string(
            input: &str,
            env: &mut HashMap<String, Box<Expr>>,
            expected: Option<f64>,
        ) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse(false).expect("failed to parse expression");
            let val = expr.eval(env);
            println!("{} evaluated to {}", expr, val);
            if let Some(expected) = expected {
                assert_eq!(
                    val.expect_val("could not evaluate expr"),
                    expected,
                    "evaluating {} did not yield {}",
                    expr,
                    expected
                );
            }
        }
        start_test("var evaluation");
        test_var_eval_on_string("a=2", &mut env, None);
        test_var_eval_on_string("a", &mut env, Some(2.0));
        test_var_eval_on_string("abcde=(1+2)*(3-4)^((2*3)^3*2)", &mut env, None);
        test_var_eval_on_string("a+abcde", &mut env, Some(5.0));
        end_test("var evaluation");
    }
}
