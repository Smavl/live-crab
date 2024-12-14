use crate::ast::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct ControlFlowGraph {
    nodes: Vec<Node>,
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

    pub fn get_nodes(&self) -> Vec<&Node> {
        self.nodes.iter().map(|n| n).collect()
    }
    pub fn get_node(&self, n: usize) -> &Node {
        if let Some(n) = self.nodes.get(n) {
            n
        } else {
            panic!("That node Could not be found")
        }
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
    // for r in res.iter() {
    //     println!("{:?}", r);
    // }
    res
}
fn handle_assignment(stmt: &Statement) -> Node {
    match stmt {
        Statement::Assignment(lvl, e) => {
            let lval = lvl.clone();
            let exp = e.clone();

            let mut node = Node::new(NodeKind::from((lval, exp)));

            // add the lvl to the def set
            if let Expr::Id(id) = *lvl.clone() {
                node.insert_def(id);
            }

            node.use_extend(e.clone().iter().collect::<HashSet<String>>());
            node
        }
        _ => panic!("Death"),
    }
}
fn handle_return(stmt: &Statement) -> Node {
    match stmt {
        Statement::Return(e) => Node::new(NodeKind::from(e.clone())),
        _ => panic!("Death"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeKind {
    Assignment(Box<Expr>, Box<Expr>),
    Condition(Box<Expr>),
    Return(Box<Expr>),
}

pub fn get_ids_from_expr(e: Box<Expr>) -> Vec<String> {
    e.iter().collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    node_kind: NodeKind,
    use_set: HashSet<String>,
    def_set: HashSet<String>,
    pred: HashSet<usize>,
    succ: HashSet<usize>,
}

impl Node {
    pub fn new(node_kind: NodeKind) -> Self {
        Node {
            node_kind,
            use_set: HashSet::new(),
            def_set: HashSet::new(),
            pred: HashSet::new(),
            succ: HashSet::new(),
        }
    }

    pub fn get_node_kind(&self) -> &NodeKind {
        &self.node_kind
    }

    pub fn use_extend(&mut self, vars: HashSet<String>) {
        self.use_set.extend(vars);
    }
    pub fn insert_use(&mut self, var: String) {
        self.use_set.insert(var);
    }
    pub fn insert_def(&mut self, var: String) {
        self.def_set.insert(var);
    }

    pub fn get_defs(&self) -> &HashSet<String> {
        &self.def_set
    }
    pub fn get_uses(&self) -> &HashSet<String> {
        &self.use_set
    }

    pub fn add_pred(&mut self, n: usize) {
        self.pred.insert(n);
    }
    pub fn add_succ(&mut self, n: usize) {
        self.succ.insert(n);
    }

    pub fn get_preds(&self) -> &HashSet<usize> {
        &self.pred
    }
    pub fn get_succs(&self) -> &HashSet<usize> {
        &self.succ
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
