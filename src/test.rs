#[cfg(test)]
mod tests {

    use crate::{
        expr::EvalEnv,
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
                (&lexer
                    .expect_token_kinds(&[kind.clone()], "while testing".to_string())
                    .unwrap()
                    .kind),
                (&kind),
                "Expected{:?} but got {:?}",
                lexer
                    .expect_token_kinds(&[kind.clone()], "while testing".to_string())
                    .unwrap()
                    .kind,
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
                .expect_token_kinds(
                    &[TokenKind::OPERANDS, &[TokenKind::OpenParen]].concat(),
                    "while testing".to_string()
                )
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
        fn test_parser_on_string(input: &str, should_fail: bool) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = match parser.parse() {
                Some(expr) => {
                    assert!(
                        !should_fail,
                        "Parsed {} from {}, expected to fail.",
                        expr, input,
                    );
                    expr
                }
                None => return assert!(should_fail, "Failed on {}, but didn't expect to", input),
            };
            println!("{}", expr);
            println!("{:#?}", expr);
        }
        test_parser_on_string("abc+1234", false);
        test_parser_on_string("1234+abc", false);
        test_parser_on_string("abc-1234", false);
        test_parser_on_string("abc*1234", false);
        test_parser_on_string("abc*1234+4321", false);
        test_parser_on_string("abc*1234+4321*420/69", false);
        test_parser_on_string("abc=1234+4321*420/69", false);
        test_parser_on_string("f(ab,c,g(q))", false);
        test_parser_on_string("abc+f(ab,c,g(q))", false);
        test_parser_on_string("123=123", true);
        test_parser_on_string("123<", true);
        test_parser_on_string("abc<", true);
        test_parser_on_string("abc*1234+4321*420/69=321+123+(12*3)", true);
        test_parser_on_string("a+(12*3", true);
        test_parser_on_string("1+2)*3", true);
        test_parser_on_string("ab 123", true);
        test_parser_on_string("f(a,b", true);
        test_parser_on_string("f(a b)", true);

        end_test("parser");
    }
    #[test]

    fn test_expr_eval() {
        start_test("expr_eval");

        fn test_expr_eval_on_string(input: &str, expected: f64) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse().expect("failed to parse expression");
            let mut env = EvalEnv::new();
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
            let expr = parser.parse().expect("failed to parse expression");
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
        let mut env = EvalEnv::new();

        fn test_var_eval_on_string(input: &str, env: &mut EvalEnv, expected: Option<f64>) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse().expect("failed to parse expression");
            println!("{}", expr);
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
        test_var_eval_on_string("b=10", &mut env, None);
        test_var_eval_on_string("c=15", &mut env, None);
        test_var_eval_on_string("d=b+c", &mut env, None);
        test_var_eval_on_string("d", &mut env, Some(25.0));
        end_test("var evaluation");
    }
    #[test]
    fn test_fun_eval() {
        let mut env = EvalEnv::new();

        fn test_fun_eval_on_string(input: &str, env: &mut EvalEnv, expected: Option<f64>) {
            let mut parser = Parser::from_string(input.to_string());
            let expr = parser.parse().expect("failed to parse expression");
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
        start_test("fun evaluation");
        test_fun_eval_on_string("f(a)=a", &mut env, None);
        test_fun_eval_on_string("f(3)", &mut env, Some(3.0));
        test_fun_eval_on_string("g(a,b)=(a+b)*(3-4)^((a*b)^3*2)", &mut env, None);
        test_fun_eval_on_string("g(2,3)", &mut env, Some(5.0));
        test_fun_eval_on_string("h(a,b)=a+b+c", &mut env, None);
        test_fun_eval_on_string("c=10", &mut env, None);
        test_fun_eval_on_string("h(1,2)", &mut env, Some(13.0));
        end_test("fun evaluation");
    }
}
