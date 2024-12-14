use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // for peeking
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        println!("Tokens from new(): {:?}", tokens);
        Parser { tokens, current: 0 }
    }

    pub fn pretty_print_program(&self, p: &Program) -> String {
        let mut sb = String::new();
        sb.push_str("Program :\n");
        sb.push_str(&self.pretty_print_statements(&p.stmts));
        sb
    }
    fn pretty_print_statements(&self, ss: &Vec<Statement>) -> String {
        let mut sb = String::new();
        for s in ss.iter() {
            sb.push_str(&self.pretty_print_statement(s));
        }
        sb
    }

    fn pretty_print_statement(&self, s: &Statement) -> String {
        let mut sb = String::new();

        match s {
            Statement::If(c, statements) => {
                let cond = self.pretty_print_expr(c);
                let stmst = self.pretty_print_statements(statements);
                sb.push_str(&format!("if ({}) {{\n{}\n}}\n", cond, stmst));
            }
            Statement::While(c, statements) => {
                let cond = self.pretty_print_expr(c);
                let stmst = self.pretty_print_statements(statements);
                sb.push_str(&format!("while ({}) {{\n{}}}\n", cond, stmst));
            }
            Statement::DoWhile(statements, c) => {
                let stmst = self.pretty_print_statements(statements);
                let cond = self.pretty_print_expr(c);
                sb.push_str(&format!("do {{\n{}}} while ({});\n", stmst, cond));
            }
            Statement::Assignment(id, e) => {
                let id = self.pretty_print_expr(id);
                let e = self.pretty_print_expr(e);
                sb.push_str(&format!("{} = {};\n", id, e));
            }
            Statement::Return(e) => {
                let e = self.pretty_print_expr(e);
                sb.push_str(&format!("return {};\n", e));
            }
        }

        sb
    }
    fn pretty_print_expr(&self, ex: &Expr) -> String {
        match ex {
            Expr::Id(id) => id.clone(),
            Expr::Int(n) => n.to_string(),
            Expr::BinOp(left, op, right) => {
                let l = self.pretty_print_expr(left);
                let o = self.pretty_print_operator(op);
                let r = self.pretty_print_expr(right);
                format!("{} {} {}", l, o, r)
            }
        }
    }

    fn pretty_print_operator(&self, op: &Operator) -> String {
        match *op {
            Operator::Plus => "+".to_string(),
            Operator::Minus => "-".to_string(),
            Operator::Mod => "%".to_string(),
            Operator::Div => "/".to_string(),
            Operator::Mult => "*".to_string(),
            Operator::LessThan => "<".to_string(),
        }
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
            // println!("Statement from loop: {:?}", e);
            stmts.push(e);
        }
        // println!("Statements from get_statements: {:?}", stmts);
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
                let cond = self.eat_expr();
                self.consume(Token::RParen);
                self.consume(Token::LBrace);
                let res = Some(Statement::While(Box::new(cond), self.get_statements()));
                self.consume(Token::RBrace);
                res
            }
            "do" => {
                self.consume(Token::LBrace);
                let body = self.get_statements();
                self.consume(Token::RBrace);
                self.consume(Token::Keyword(String::from("while")));
                self.consume(Token::LParen);
                let cond = self.eat_expr();
                self.consume(Token::RParen);
                self.consume(Token::Semicolon);
                Some(Statement::DoWhile(body, Box::new(cond)))
            }
            _ => None,
        }
    }

    fn eat_assignment(&mut self) -> Option<Statement> {
        let id = match self.peek() {
            Some(Token::Id(id)) => {
                // println!("id: {}", id);
                id.clone()
            }
            e => {
                // println!("got: {:?}", e);
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
            match op.clone() {
                t_op @ (Token::Div
                | Token::Mod
                | Token::Plus
                | Token::Minus
                | Token::LessThan
                | Token::Mult) => {
                    self.consume(t_op.clone());
                    let right = self.left_exp();
                    left = Expr::BinOp(Box::new(left), token_to_operator(&t_op), Box::new(right));
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
        // println!("Consuming: {:?}", tk);
        if Some(&tk) == self.peek() {
            self.current += 1;
        } else {
            for (idx, t) in self.tokens.iter().enumerate() {
                println!("Token {idx}: {:?}", t);
            }
            panic!("Could not eat {:?}", tk)
        }
    }
}
fn token_to_operator(t: &Token) -> Operator {
    match t {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::Mult => Operator::Mult,
        Token::Div => Operator::Div,
        Token::Mod => Operator::Mod,
        Token::LessThan => Operator::LessThan,
        _ => panic!("Not an operator"),
    }
}
