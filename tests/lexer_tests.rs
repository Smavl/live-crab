use live_crab::lexer::Lexer;
use live_crab::lexer::Token;

mod test_utils;


#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    // advance tests
    #[test]
    fn advance_first() {
        let s = "a = 3;";
        let mut lexer = Lexer::new(s);
        assert_eq!(
            lexer.get_current(),
            Some('a'),
            "Lexer did not Init correctly"
        );
    }
    #[test]
    fn advance_simple() {
        let s = "a = 3;";
        let mut lexer = Lexer::new(s);
        assert_eq!(lexer.get_current(), Some('a'));
        lexer.advance();
        assert_eq!(lexer.get_current(), Some(' '));
        lexer.advance();
        assert_eq!(lexer.get_current(), Some('='));
        lexer.advance();
        assert_eq!(lexer.get_current(), Some(' '));
        lexer.advance();
        assert_eq!(lexer.get_current(), Some('3'));
        lexer.advance();
        assert_eq!(lexer.get_current(), Some(';'));
    }
    // Token test
    #[test]
    fn doesnt_lex_whitespace() {
        let s = " ;\n\t\r";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Semicolon);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_semicolon() {
        let s = ";";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Semicolon);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_plus() {
        let s = "+";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Plus);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_equals() {
        let s = "=";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Equals);
        assert_eq!(got, &want);
    }
    // keywords
    #[test]
    fn lex_return() {
        let s = "return";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("return")));
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_if() {
        let s = "if";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("if")));
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_do_keyword() {
        let s = "do";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("do")));
        assert_eq!(got, &want);
    }
    // Identifiers
    #[test]
    fn lex_id_a() {
        let s = "a";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Id(String::from("a")));
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_id_an() {
        let s = "a123";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Id(String::from("a123")));
        assert_eq!(got, &want);
    }

    // control floow
    #[test]
    fn lex_if_with_cond() {
        let s = "if ( a < 10 )";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("if")));
        want.push(Token::LParen);
        want.push(Token::Id(String::from("a")));
        want.push(Token::LessThan);
        want.push(Token::Int(10));
        want.push(Token::RParen);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_if_with_cond_body() {
        let s = "if ( a < 10 ) { return a; }";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("if")));
        want.push(Token::LParen);
        want.push(Token::Id(String::from("a")));
        want.push(Token::LessThan);
        want.push(Token::Int(10));
        want.push(Token::RParen);
        want.push(Token::LBrace);
        want.push(Token::Keyword(String::from("return")));
        want.push(Token::Id(String::from("a")));
        want.push(Token::Semicolon);
        want.push(Token::RBrace);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_while_loop() {
        let s = "while ( a < 10 ) { a = a + 1; }";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("while")));
        want.push(Token::LParen);
        want.push(Token::Id(String::from("a")));
        want.push(Token::LessThan);
        want.push(Token::Int(10));
        want.push(Token::RParen);
        want.push(Token::LBrace);
        want.push(Token::Id(String::from("a")));
        want.push(Token::Equals);
        want.push(Token::Id(String::from("a")));
        want.push(Token::Plus);
        want.push(Token::Int(1));
        want.push(Token::Semicolon);
        want.push(Token::RBrace);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_do_while_loop() {
        let s = "do { a = a + 1; } while ( a < 10 ); ";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Keyword(String::from("do")));
        want.push(Token::LBrace);
        want.push(Token::Id(String::from("a")));
        want.push(Token::Equals);
        want.push(Token::Id(String::from("a")));
        want.push(Token::Plus);
        want.push(Token::Int(1));
        want.push(Token::Semicolon);
        want.push(Token::RBrace);
        want.push(Token::Keyword(String::from("while")));
        want.push(Token::LParen);
        want.push(Token::Id(String::from("a")));
        want.push(Token::LessThan);
        want.push(Token::Int(10));
        want.push(Token::RParen);
        want.push(Token::Semicolon);
        assert_eq!(got, &want);
    }

    // Int
    #[test]
    fn lex_int() {
        let s = "42";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Int(42));
        assert_eq!(got, &want);
    }

    // Examples
    #[test]
    fn lex_simple_assignment() {
        let s = "a = 42;";
        let lexer = Lexer::new(s);
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Id(String::from("a")));
        want.push(Token::Equals);
        want.push(Token::Int(42));
        want.push(Token::Semicolon);
        assert_eq!(got, &want);
    }
    #[test]
    fn lex_simple_example1() {
        let file = get_str_from_path("examples/s1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Id(String::from("a")));
        want.push(Token::Equals);
        want.push(Token::Int(2));
        want.push(Token::Semicolon);

        want.push(Token::Id(String::from("b")));
        want.push(Token::Equals);
        want.push(Token::Int(3));
        want.push(Token::Semicolon);

        want.push(Token::Keyword(String::from("return")));
        want.push(Token::Id(String::from("a")));
        want.push(Token::Semicolon);
        assert_eq!(got, &want, "Got: {:?}\n\n", got);
    }
    #[test]
    fn lex_loop_example1() {
        // Code in examples/loop1
        //i = 0;
        //while (i < 9) {
        //i = i + 1;
        //}
        //return i;
        let file = get_str_from_path("examples/loop1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let got = &lexer.tokenize();
        let mut want = Vec::new();
        want.push(Token::Id(String::from("i")));
        want.push(Token::Equals);
        want.push(Token::Int(0));
        want.push(Token::Semicolon);

        want.push(Token::Keyword(String::from("while")));
        want.push(Token::LParen);
        want.push(Token::Id(String::from("i")));
        want.push(Token::LessThan);
        want.push(Token::Int(9));
        want.push(Token::RParen);
        want.push(Token::LBrace);
        want.push(Token::Id(String::from("i")));
        want.push(Token::Equals);
        want.push(Token::Id(String::from("i")));
        want.push(Token::Plus);
        want.push(Token::Int(1));
        want.push(Token::Semicolon);
        want.push(Token::RBrace);

        want.push(Token::Keyword(String::from("return")));
        want.push(Token::Id(String::from("i")));
        want.push(Token::Semicolon);
        assert_eq!(got, &want, "Got: {:?}\n\n", got);
    }
}
