use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Id(String),
    Int(i32),
    BinOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    LessThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Assignment(Box<Expr>, Box<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Vec<Statement>),
    While(Box<Expr>, Vec<Statement>),
    DoWhile(Vec<Statement>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub stmts: Vec<Statement>,
    // pub variables: Vec<Expr::Id(String)>,
}

impl Program {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Program { stmts }
    }

    // pub fn
}
