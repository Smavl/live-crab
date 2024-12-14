use crate::ast::*;
use crate::parser::*;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct ControlFlowGraph {
    nodes: Vec<Node>,
}

impl Display for ControlFlowGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for n in self.nodes.iter() {
            match &n.node_kind {
                NodeKind::Return(e) => {
                    let mut ret = String::new();
                    ret.push_str("return ");
                    ret.push_str(format!("{};\n", e.clone()).as_str());
                    res.push_str(ret.as_str())
                }
                NodeKind::Assignment(lvl, e) => {
                    let mut ret = String::new();
                    ret.push_str(format!("{} = {};\n", lvl.clone(), e.clone()).as_str());
                    res.push_str(ret.as_str())
                }
                NodeKind::Condition(e) => {
                    let mut ret = String::new();
                    ret.push_str(format!("if {}\n", e.clone()).as_str());
                    res.push_str(ret.as_str())
                }
                e => panic!("aws {:?}", e),
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

impl ControlFlowGraph {
    pub fn new() -> Self {
        ControlFlowGraph { nodes: Vec::new() }
    }

    pub fn from(p: &Program) -> Self {
        let mut cfg = ControlFlowGraph::new();
        cfg.nodes = flatten_program(p);
        cfg
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

fn flatten_program(p: &Program) -> Vec<Node> {
    let flattened_stms = p
        .stmts
        .iter()
        .map(|s| flatten_statements_with_bodies(s))
        .flatten()
        .collect::<Vec<Node>>();

    flattened_stms
}
fn flatten_statements_with_bodies(stmt: &Statement) -> Vec<Node> {
    let mut res = Vec::new();
    match stmt {
        a @ Statement::Assignment(_, _) => res.push(handle_assignment(a)),
        r @ Statement::Return(_) => res.push(handle_return(r)),
        Statement::If(cond, body) => {
            let cond = Node::new(NodeKind::from(cond.clone()));

            res.push(cond);

            let body = body
                .iter()
                .map(|s| flatten_statements_with_bodies(s))
                .flatten()
                .collect::<Vec<Node>>();

            res.extend(body)
        }
        Statement::DoWhile(body, cond) => {
            let body = body
                .iter()
                .map(|s| flatten_statements_with_bodies(s))
                .flatten()
                .collect::<Vec<Node>>();

            let cond = Node::new(NodeKind::from(cond.clone()));

            res.extend(body);
            res.push(cond);
        }
        e => panic!("Dun goofed!: {:?}", e),
    }
    for r in res.iter() {
        println!("{:?}", r);
    }
    res
}
fn handle_assignment(stmt: &Statement) -> Node {
    match stmt {
        Statement::Assignment(lvl, e) => Node::new(NodeKind::from((lvl.clone(), e.clone()))),
        _ => panic!("Death"),
    }
}
fn handle_return(stmt: &Statement) -> Node {
    match stmt {
        Statement::Return(e) => Node::new(NodeKind::from(e.clone())),
        _ => panic!("Death"),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Assignment(Box<Expr>, Box<Expr>),
    Condition(Box<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    node_kind: NodeKind,
}

impl Node {
    pub fn new(node_kind: NodeKind) -> Self {
        Node { node_kind }
    }
}

impl From<(Box<Expr>, Box<Expr>)> for NodeKind {
    fn from((lvl, exp): (Box<Expr>, Box<Expr>)) -> Self {
        NodeKind::Assignment(lvl, exp)
    }
}

impl From<Box<Expr>> for NodeKind {
    fn from(exp: Box<Expr>) -> Self {
        match *exp {
            Expr::BinOp(l, op, r) => match op {
                Operator::LessThan => NodeKind::Condition(Box::new(Expr::BinOp(l, op, r))),
                opr @ (Operator::Plus | Operator::Minus | Operator::Mult) => {
                    NodeKind::Return(Box::new(Expr::BinOp(l, opr, r)))
                }
                _ => panic!("upson"),
            },
            Expr::Id(id) => NodeKind::Return(Box::new(Expr::Id(id))),
            e => panic!("oppsi, {:?}", e),
        }
    }
}
