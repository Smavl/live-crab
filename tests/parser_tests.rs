use live_crab::ast::*;
use live_crab::lexer::Lexer;
use live_crab::parser::Parser;

mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    #[test]
    fn parse_simple_assignment() {
        // with fake it code
        let s = "a = 42;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![Statement::Assignment(
            Box::new(Expr::Id("a".to_string())),
            Box::new(Expr::Int(42)),
        )];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_simple_assignment2() {
        let s = "a = 68 +1 ;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![Statement::Assignment(
            Box::new(Expr::Id("a".to_string())),
            Box::new(create_binop_lit(68, Operator::Plus, 1)),
        )];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_three_add() {
        let s = "a = 39 + 1 +2 ;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![Statement::Assignment(
            Box::new(Expr::Id("a".to_string())),
            Box::new(create_binop_rlit(
                create_binop_lit(39, Operator::Plus, 1),
                Operator::Plus,
                2,
            )),
        )];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_simple_example1() {
        // a = 2; b = 3; return a;
        let file = get_str_from_path("examples/s1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![
            Statement::Assignment(Box::new(Expr::Id("a".to_string())), Box::new(Expr::Int(2))),
            Statement::Assignment(Box::new(Expr::Id("b".to_string())), Box::new(Expr::Int(3))),
            Statement::Return(Box::new(Expr::Id("a".to_string()))),
        ];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_while_loop() {
        // while (i < 9) {
        //  i = i + 1;
        // }
        let s = "while (i < 9) { i = i + 1; }";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![
            // while (i < 9) {
            Statement::While(
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::LessThan,
                    Box::new(Expr::Int(9)),
                )),
                // body
                vec![
                    // i = i + 1;
                    Statement::Assignment(
                        Box::new(Expr::Id("i".to_string())),
                        Box::new(create_binop_rlit(
                            Expr::Id("i".to_string()),
                            Operator::Plus,
                            1,
                        )),
                    ),
                ],
            ),
        ];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_do_while_loop_() {
        // Code from Examples/loop1 :
        // i = 0;
        // do {
        //  i = i + 1;
        // }
        // while (i < 9);
        // return i;
        let file = get_str_from_path("examples/do_while").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![
            // i = 0;
            Statement::Assignment(Box::new(Expr::Id("i".to_string())), Box::new(Expr::Int(0))),
            // do {
            Statement::DoWhile(
                // body
                vec![
                    // i = i + 1;
                    Statement::Assignment(
                        Box::new(Expr::Id("i".to_string())),
                        Box::new(create_binop_rlit(
                            Expr::Id("i".to_string()),
                            Operator::Plus,
                            1,
                        )),
                    ),
                    Statement::Assignment(
                        Box::new(Expr::Id("i".to_string())),
                        Box::new(create_binop_rlit(
                            Expr::Id("i".to_string()),
                            Operator::Plus,
                            1,
                        )),
                    ),
                ],
                // while (i < 9);
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::LessThan,
                    Box::new(Expr::Int(9)),
                )),
            ),
            Statement::Return(Box::new(Expr::Id("i".to_string()))),
        ];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn parser_while_loop_example1() {
        // Code from Examples/loop1 :
        // i = 0;
        // while (i < 9) {
        //  i = i + 1;
        // }
        // return i;
        let file = get_str_from_path("examples/loop1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let got = parser.parse();
        let want_vec = vec![
            // i = 0;
            Statement::Assignment(Box::new(Expr::Id("i".to_string())), Box::new(Expr::Int(0))),
            // while (i < 9) {
            Statement::While(
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::LessThan,
                    Box::new(Expr::Int(9)),
                )),
                // body
                vec![
                    // i = i + 1;
                    Statement::Assignment(
                        Box::new(Expr::Id("i".to_string())),
                        Box::new(create_binop_rlit(
                            Expr::Id("i".to_string()),
                            Operator::Plus,
                            1,
                        )),
                    ),
                ],
            ),
            Statement::Return(Box::new(Expr::Id("i".to_string()))),
        ];
        let want = Program::new(want_vec);
        assert_eq!(got, want, "\nGot: {}\n\nWant: {}\n\n", &got, &want);
    }
}
