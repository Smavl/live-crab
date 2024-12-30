// Overall structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub stmts: Vec<Statement>,
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

pub struct ExprIdIterator {
    seq: Vec<Box<Expr>>,
}

// Implementations
impl Program {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Program { stmts }
    }
}

impl ExprIdIterator {
    fn new(expr: Box<Expr>) -> Self {
        ExprIdIterator { seq: vec![expr] }
    }
}
impl Iterator for ExprIdIterator {
    // HACK: id is just a string
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_iterator() {
        let got_exp = 
            Box::new(Expr::BinOp(
                Box::new(Expr::Id("a".to_string())),
                Operator::Plus,
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("b".to_string())),
                    Operator::Plus,
                    Box::new(Expr::Id("c".to_string())),
                    )           
                )
            )
        );

        let mut got_it = got_exp.iter();
        assert_eq!(got_it.next(), Some(String::from("a")));
        assert_eq!(got_it.next(), Some(String::from("b")));
        assert_eq!(got_it.next(), Some(String::from("c")));
        assert_eq!(got_it.next(), None);
    }
}
