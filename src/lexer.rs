use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Id(String),
    Int(i32),
    Equals,
    LessThan,
    GreaterThan,
    Keyword(String),
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Colon,
}

fn char_to_token(c: char) -> Option<Token> {
    match c {
        ';' => Some(Token::Semicolon),
        '=' => Some(Token::Equals),
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '*' => Some(Token::Mult),
        '/' => Some(Token::Div),
        '%' => Some(Token::Mod),
        '(' => Some(Token::LParen),
        ')' => Some(Token::RParen),
        '<' => Some(Token::LessThan),
        '{' => Some(Token::LBrace),
        '}' => Some(Token::RBrace),
        // ':' => Some(Token::Colon),
        _ => None,
    }
}
fn string_to_token(s: &str) -> Option<Token> {
    match s {
        "if" => Some(Token::Keyword(String::from("if"))),
        "return" => Some(Token::Keyword(String::from("return"))),
        "while" => Some(Token::Keyword(String::from("while"))),
        "break" => Some(Token::Keyword(String::from("break"))),
        "do" => Some(Token::Keyword(String::from("do"))),
        s => Some(Token::Id(String::from(s))),
    }
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: None,
        };
        lexer.advance();
        lexer
    }
    pub fn advance(&mut self) {
        self.current = self.input.next();
    }

    pub fn get_current(&mut self) -> Option<char> {
        self.current
    }

    // Returns the next token if possible
    // Whitespace are non-tokens
    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current {
            match c {
                c if c.is_whitespace() => {
                    self.advance();
                    // we only return None for errors
                    continue;
                }
                // Handle identifiers and keywords
                // - Should ensure that keyword
                //   starts with a letter
                c if c.is_alphabetic() => return self.consume_keyword_or_id(),
                c if c.is_numeric() => return self.consume_numeric(),
                // del @ (';' | '=' | '+' | '(' | ')' | '{' | '}') => {
                del if !c.is_alphanumeric() => {
                    self.advance();
                    return char_to_token(del);
                }
                _ => return None,
            }
        }
        None
    }

    fn consume_numeric(&mut self) -> Option<Token> {
        let mut n = String::new();
        // collect chars until non alphanumeric
        while let Some(c) = self.current {
            if c.is_numeric() {
                // identifier.push(c);
                n.push(c);
                self.advance();
                continue;
            } else {
                break;
            }
        }
        if let Some(i) = n.parse().ok() {
            Some(Token::Int(i))
        } else {
            None
        }
    }

    fn consume_keyword_or_id(&mut self) -> Option<Token> {
        let mut identifier = String::new();
        // collect chars until non alphanumeric
        while let Some(c) = self.current {
            if c.is_alphanumeric() {
                // identifier.push(c);
                identifier.push(c);
                self.advance();
                continue;
            } else {
                break;
            }
        }

        match identifier.as_str() {
            s => return string_to_token(s),
            // keyword @ ("return" | "if") => return string_to_token(keyword),
            // id => return string_to_token(id),
        };
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut res = Vec::new();
        while let Some(t) = self.next_token() {
            res.push(t);
        }

        res
    }
}
