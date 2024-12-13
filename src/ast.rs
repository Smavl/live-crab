#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Id(String),
    Int(i32),
    BinOp(Box<Expr>, Box<Operator>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Rem,
    LessThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Assignment(Box<Expr>, Box<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Vec<Statement>),
    While(Box<Expr>, Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    stmts: Vec<Statement>,
}

impl Program {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Program { stmts }
    }
}
