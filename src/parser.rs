use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // for peeking
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        println!("Tokens from new(): {:?}", tokens);
        // tokens.reverse();
        Parser { tokens, current: 0 }
    }

    // utility
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn parse(&mut self) -> Program {
        let stmts = self.get_statements();
        Program::new(stmts)
    }

    fn get_statements(&mut self) -> Vec<Statement> {
        let mut stmts = Vec::new();
        while let Some(e) = self.next_statement() {
            println!("Statement from loop: {:?}", e);
            stmts.push(e);
        }
        println!("Statements from get_statements: {:?}", stmts);
        stmts
    }
    pub fn next_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            // Some(t) => {
            //     println!("Next_statment starts with token: {:?}", t);
            //     return self.eat_assignment();
            // }
            Some(Token::Id(_)) => return self.eat_assignment(),
            Some(Token::Keyword(s)) => {
                let s = s.clone();
                return self.eat_keyword(s);
            }
            _ => None,
        }
    }

    fn eat_keyword(&mut self, kw: String) -> Option<Statement> {
        println!("Eating keyword: {}", kw);
        self.consume(Token::Keyword(kw.to_string()));
        match kw.as_str() {
            "return" => {
                let e = self.eat_expr();
                self.consume(Token::Semicolon);
                Some(Statement::Return(Box::new(e)))
            }
            "if" => {
                self.consume(Token::LParen);
                let e = self.eat_expr();
                self.consume(Token::RParen);
                self.consume(Token::LBrace);
                let res = Some(Statement::If(Box::new(e), self.get_statements()));
                self.consume(Token::RBrace);
                res
            }
            "while" => {
                self.consume(Token::LParen);
                let e = self.eat_expr();
                self.consume(Token::RParen);
                self.consume(Token::LBrace);
                let res = Some(Statement::While(Box::new(e), self.get_statements()));
                self.consume(Token::RBrace);
                res
            }
            _ => None,
        }
    }

    fn eat_assignment(&mut self) -> Option<Statement> {
        let id = match self.peek() {
            Some(Token::Id(id)) => {
                println!("id: {}", id);
                id.clone()
            }
            e => {
                println!("got: {:?}", e);
                panic!("Was not an id")
            }
        };

        self.consume(Token::Id(id.clone()));
        self.consume(Token::Equals);
        let e = self.eat_expr();
        self.consume(Token::Semicolon);

        Some(Statement::Assignment(
            Box::new(Expr::Id(id.clone())),
            Box::new(e),
        ))
    }

    fn eat_expr(&mut self) -> Expr {
        let mut left = self.left_exp();

        while let Some(op) = self.peek() {
            match op {
                Token::Plus => {
                    self.consume(Token::Plus);
                    let right = self.left_exp();
                    left = Expr::BinOp(Box::new(left), Box::new(Operator::Plus), Box::new(right));
                }
                Token::Minus => {
                    self.consume(Token::Minus);
                    let right = self.left_exp();
                    left = Expr::BinOp(Box::new(left), Box::new(Operator::Minus), Box::new(right));
                }
                Token::LessThan => {
                    self.consume(Token::LessThan);
                    let right = self.left_exp();
                    left = Expr::BinOp(
                        Box::new(left),
                        Box::new(Operator::LessThan),
                        Box::new(right),
                    );
                }
                _ => break, // No more binary operators, break out of loop
            }
        }

        left
    }
    fn left_exp(&mut self) -> Expr {
        let t = self.peek();

        match t {
            Some(Token::Int(n)) => {
                // Consume the integer token
                let n = n.clone(); // to not borrow too much
                self.consume(Token::Int(n));
                Expr::Int(n)
            }
            Some(Token::Id(id)) => {
                let id = id.clone();
                self.consume(Token::Id(id.clone()));
                Expr::Id(id)
            }
            _ => panic!("Unexpected token in left expression"),
        }
    }

    fn consume(&mut self, tk: Token) {
        println!("Consuming: {:?}", tk);
        if Some(&tk) == self.peek() {
            self.current += 1;
        } else {
            panic!("Could not eat {:?}", tk)
        }
    }
}
