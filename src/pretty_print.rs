use crate::ast::*;
use crate::liveness::*;
use std::fmt::format;
use std::fmt::Display;

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

        for (idx, n) in self.get_nodes().iter().enumerate() {
            match (idx, n.get_node_kind()) {
                (_, NodeKind::Return(e)) => {
                    res.push_str(&format!("{idx}: return {};\n", e));
                }
                (_,NodeKind::Assignment(lvl, e)) => {
                    res.push_str(&format!("{idx}: {} = {};\n", lvl, e,));
                }
                (_,NodeKind::Condition(e)) => {
                    res.push_str(&format!("{idx}: if {}\n", e));
                }
            }
            res.push_str(
                &format!("\tdef: {:?}, use: {:?}\n",
                    n.get_defs(),
                    n.get_uses()
                ));
            res.push_str(
                &format!("\tpred: {:?}, succ: {:?}\n",
                    n.get_preds(),
                    n.get_succs()
                ));
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
