#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Id(String),
    Int(i32),
    BinOp(Box<Expr>, Operator, Box<Expr>),
}

pub struct ExprIdIterator {
    seq: Vec<Box<Expr>>,
}
impl ExprIdIterator {
    fn new(expr: Box<Expr>) -> Self {
        ExprIdIterator { seq: vec![expr] }
    }
}
impl Iterator for ExprIdIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(expr) = self.seq.pop() {
            match *expr {
                Expr::Id(id) => return Some(id),
                Expr::Int(_) => continue,
                Expr::BinOp(l, _, r) => {
                    // Order, left branch should be next ;)
                    self.seq.push(r);
                    self.seq.push(l);
                }
            }
        }
        None
    }
}

impl Expr {
    pub fn iter(self: Box<Self>) -> ExprIdIterator {
        ExprIdIterator::new(self)
    }
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
