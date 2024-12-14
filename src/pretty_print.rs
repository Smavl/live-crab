use crate::ast::*;
use crate::liveness::*;
use crate::parser::*;
use std::fmt::Display;

// impl Program {
//     pub fn pretty_print_program(&self, p: &Program) -> String {
//         let mut sb = String::new();
//         sb.push_str("Program :\n");
//         sb.push_str(&self.pretty_print_statements(&p.stmts));
//         sb
//     }
//     fn pretty_print_statements(&self, ss: &Vec<Statement>) -> String {
//         let mut sb = String::new();
//         for s in ss.iter() {
//             sb.push_str(&self.pretty_print_statement(s));
//         }
//         sb
//     }

//     fn pretty_print_statement(&self, s: &Statement) -> String {
//         let mut sb = String::new();

//         match s {
//             Statement::If(c, statements) => {
//                 let cond = self.pretty_print_expr(c);
//                 let stmst = self.pretty_print_statements(statements);
//                 sb.push_str(&format!("if ({}) {{\n{}\n}}\n", cond, stmst));
//             }
//             Statement::While(c, statements) => {
//                 let cond = self.pretty_print_expr(c);
//                 let stmst = self.pretty_print_statements(statements);
//                 sb.push_str(&format!("while ({}) {{\n{}}}\n", cond, stmst));
//             }
//             Statement::DoWhile(statements, c) => {
//                 let stmst = self.pretty_print_statements(statements);
//                 let cond = self.pretty_print_expr(c);
//                 sb.push_str(&format!("do {{\n{}}} while ({});\n", stmst, cond));
//             }
//             Statement::Assignment(id, e) => {
//                 let id = self.pretty_print_expr(id);
//                 let e = self.pretty_print_expr(e);
//                 sb.push_str(&format!("{} = {};\n", id, e));
//             }
//             Statement::Return(e) => {
//                 let e = self.pretty_print_expr(e);
//                 sb.push_str(&format!("return {};\n", e));
//             }
//         }

//         sb
//     }

//     fn pretty_print_expr(&self, ex: &Expr) -> String {
//         match ex {
//             Expr::Id(id) => id.clone(),
//             Expr::Int(n) => n.to_string(),
//             Expr::BinOp(left, op, right) => {
//                 let l = self.pretty_print_expr(left);
//                 let o = self.pretty_print_operator(op);
//                 let r = self.pretty_print_expr(right);
//                 format!("{} {} {}", l, o, r)
//             }
//         }
//     }

//     fn pretty_print_operator(&self, op: &Operator) -> String {
//         match *op {
//             Operator::Plus => "+".to_string(),
//             Operator::Minus => "-".to_string(),
//             Operator::Mod => "%".to_string(),
//             Operator::Div => "/".to_string(),
//             Operator::Mult => "*".to_string(),
//             Operator::LessThan => "<".to_string(),
//         }
//     }
// }

// impl Display for Program {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.pretty_print_program(self))
//     }
// }

// impl Display for ControlFlowGraph {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut res = String::new();

//         for n in self.get_nodes().iter() {
//             match n.get_node_kind() {
//                 NodeKind::Return(e) => {
//                     let mut ret = String::new();
//                     ret.push_str(&format!("return {};\n", e.clone()).as_str());
//                     res.push_str(ret.as_str())
//                 }
//                 NodeKind::Assignment(lvl, e) => {
//                     let mut ret = String::new();
//                     ret.push_str(&format!("{} = {};\n", lvl.clone(), e.clone()).as_str());
//                     res.push_str(ret.as_str())
//                 }
//                 NodeKind::Condition(e) => {
//                     let mut ret = String::new();
//                     ret.push_str(&format!("if {}\n", *e.as_str()));
//                     res.push_str(ret.as_str())
//                 } // e => panic!("aws {:?}", e),
//             }
//         }
//         write!(f, "{}", res)
//     }
// }

// impl Display for Expr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", pretty_print_expr(self))
//     }
// }
// fn pretty_print_expr(ex: &Expr) -> String {
//     match ex {
//         Expr::Id(id) => id.clone(),
//         Expr::Int(n) => n.to_string(),
//         Expr::BinOp(left, op, right) => {
//             let l = pretty_print_expr(left);
//             let o = pretty_print_operator(op);
//             let r = pretty_print_expr(right);
//             format!("{} {} {}", l, o, r)
//         }
//     }
// }

// fn pretty_print_operator(op: &Operator) -> String {
//     match *op {
//         Operator::Plus => "+".to_string(),
//         Operator::Minus => "-".to_string(),
//         Operator::Mod => "%".to_string(),
//         Operator::Div => "/".to_string(),
//         Operator::Mult => "*".to_string(),
//         Operator::LessThan => "<".to_string(),
//     }
// }

// use crate::ast::*;
// use crate::liveness::*;
// use crate::parser::*;
// use std::fmt::Display;

impl Program {
    pub fn pretty_print_program(p: &Program) -> String {
        let mut sb = String::new();
        sb.push_str("Program :\n");
        sb.push_str(&Self::pretty_print_statements(&p.stmts));
        sb
    }

    fn pretty_print_statements(ss: &[Statement]) -> String {
        let mut sb = String::new();
        for s in ss.iter() {
            sb.push_str(&Self::pretty_print_statement(s));
        }
        sb
    }

    fn pretty_print_statement(s: &Statement) -> String {
        let mut sb = String::new();

        match s {
            Statement::If(c, statements) => {
                let cond = Self::pretty_print_expr(c);
                let stmst = Self::pretty_print_statements(statements);
                sb.push_str(&format!("if ({}) {{\n{}\n}}\n", cond, stmst));
            }
            Statement::While(c, statements) => {
                let cond = Self::pretty_print_expr(c);
                let stmst = Self::pretty_print_statements(statements);
                sb.push_str(&format!("while ({}) {{\n{}}}\n", cond, stmst));
            }
            Statement::DoWhile(statements, c) => {
                let stmst = Self::pretty_print_statements(statements);
                let cond = Self::pretty_print_expr(c);
                sb.push_str(&format!("do {{\n{}}} while ({});\n", stmst, cond));
            }
            Statement::Assignment(id, e) => {
                let id = Self::pretty_print_expr(id);
                let e = Self::pretty_print_expr(e);
                sb.push_str(&format!("{} = {};\n", id, e));
            }
            Statement::Return(e) => {
                let e = Self::pretty_print_expr(e);
                sb.push_str(&format!("return {};\n", e));
            }
        }

        sb
    }

    fn pretty_print_expr(ex: &Expr) -> String {
        match ex {
            Expr::Id(id) => id.clone(),
            Expr::Int(n) => n.to_string(),
            Expr::BinOp(left, op, right) => {
                let l = Self::pretty_print_expr(left);
                let o = Self::pretty_print_operator(op);
                let r = Self::pretty_print_expr(right);
                format!("{} {} {}", l, o, r)
            }
        }
    }

    fn pretty_print_operator(op: &Operator) -> String {
        match *op {
            Operator::Plus => "+".to_string(),
            Operator::Minus => "-".to_string(),
            Operator::Mod => "%".to_string(),
            Operator::Div => "/".to_string(),
            Operator::Mult => "*".to_string(),
            Operator::LessThan => "<".to_string(),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Program::pretty_print_program(self))
    }
}

impl Display for ControlFlowGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for n in self.get_nodes().iter() {
            match n.get_node_kind() {
                NodeKind::Return(e) => {
                    res.push_str(&format!("return {};\n", e));
                }
                NodeKind::Assignment(lvl, e) => {
                    res.push_str(&format!("{} = {};\n", lvl, e));
                }
                NodeKind::Condition(e) => {
                    res.push_str(&format!("if {}\n", e));
                }
            }
        }
        write!(f, "{}", res)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", pretty_print_expr(self))
    }
}

fn pretty_print_expr(ex: &Expr) -> String {
    match ex {
        Expr::Id(id) => id.clone(),
        Expr::Int(n) => n.to_string(),
        Expr::BinOp(left, op, right) => {
            let l = pretty_print_expr(left);
            let o = pretty_print_operator(op);
            let r = pretty_print_expr(right);
            format!("{} {} {}", l, o, r)
        }
    }
}

fn pretty_print_operator(op: &Operator) -> String {
    match *op {
        Operator::Plus => "+".to_string(),
        Operator::Minus => "-".to_string(),
        Operator::Mod => "%".to_string(),
        Operator::Div => "/".to_string(),
        Operator::Mult => "*".to_string(),
        Operator::LessThan => "<".to_string(),
    }
}