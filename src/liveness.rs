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

struct FlattenerState {
    nodes: Vec<Node>,
    current_offset: usize,
}

impl FlattenerState {
    fn new() -> Self {
        FlattenerState {
            nodes: Vec::new(),
            current_offset: 0,
        }
    }

    fn add_node(&mut self, mut node: Node) {
        let cur_off = self.get_offset();
        // if the the node has no succs and is not a return statements
        if node.get_succs().is_empty() {
            if let NodeKind::Return(_) = node.get_node_kind() {
            } else {
                // add the next node ass a succ
                node.add_succ(self.get_offset() + 1);
            }
        }

        for ele in node.get_succs() {
            if let Some(_) = self.nodes.get(*ele) {
                self.nodes[*ele].add_pred(cur_off);
            }
        }

        // If the node is a succ to any node,
        // and add it to the node's pred set
        for (idx, n) in self.nodes.iter().enumerate() {
            if n.get_succs().contains(&cur_off) {
                node.add_pred(idx);
            }
        }

        self.nodes.push(node);
        self.inc_offset();
    }

    fn get_offset(&self) -> usize {
        self.current_offset
    }
    fn inc_offset(&mut self) {
        self.current_offset += 1;
    }
}

fn flatten_program(p: &Program) -> Vec<Node> {
    let mut flat_state = FlattenerState::new();

    flatten_statements(&mut flat_state, p.stmts.iter().collect());

    if let Some(last_node) = flat_state.nodes.last_mut() {
        last_node.clear_succ();
    }

    flat_state.nodes
}
fn flatten_statements(state: &mut FlattenerState, stmts: Vec<&Statement>) {
    for stmt in stmts {
        match stmt {
            a @ Statement::Assignment(_, _) => {
                let node = handle_assignment(a);

                state.add_node(node);
            }
            r @ Statement::Return(_) => {
                let node = handle_return(r);
                state.add_node(node);
            }
            Statement::If(cond, body) => {
                let body_start = state.get_offset() + 1;

                let mut cond_node = Node::new(NodeKind::Condition(cond.clone()));

                let mut body_flat_state = FlattenerState::new();

                body_flat_state.current_offset = body_start;

                flatten_statements(&mut body_flat_state, body.iter().collect());
                let body_end = body_flat_state.nodes.len();

                cond_node.add_succ(body_start);
                cond_node.add_succ(body_start + body_end);

                state.add_node(cond_node);

                for bn in body_flat_state.nodes {
                    state.add_node(bn);
                }
            }
            Statement::DoWhile(body, cond) => {
                let body_start = state.get_offset();

                let mut body_flat_state = FlattenerState::new();
                body_flat_state.current_offset = body_start;
                flatten_statements(&mut body_flat_state, body.iter().collect());

                let body_end = body_start + body_flat_state.nodes.len();

                for bn in body_flat_state.nodes {
                    state.add_node(bn);
                }

                let mut cond_node = Node::new(NodeKind::Condition(cond.clone()));

                // Cond node goes to start of loop if true
                cond_node.add_succ(body_start);
                // Cond node next node if false
                cond_node.add_succ(body_end + 1);

                state.add_node(cond_node);
            }
            Statement::While(cond, body) => {
                // Handle body
                let mut body_flat_state = FlattenerState::new();
                let body_start = state.get_offset() + 1;
                body_flat_state.current_offset = body_start; // feed offset to body

                flatten_statements(&mut body_flat_state, body.iter().collect());
                let body_len = body_flat_state.nodes.len();

                // init cond node and modify
                let mut cond_node = Node::new(NodeKind::Condition(cond.clone()));
                cond_node.add_succ(body_start);
                cond_node.add_succ(body_start + body_len);

                state.add_node(cond_node);

                // add nodes from body to the this state
                for bn in body_flat_state.nodes {
                    state.add_node(bn);
                }
            }
            // this is merely keep error recovery when additions are made
            _ => panic!("Not implemented yet"),
        }
    }
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
            // According to the compiler book, the sets below
            // should stored in the CFG to improve modulatity
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

    pub fn set_pred(&mut self, nv: Vec<usize>) {
        self.pred.clear();
        let _ = nv.iter().map(|p| self.add_pred(*p));
    }
    pub fn set_succ(&mut self, nv: Vec<usize>) {
        self.succ.clear();
        let _ = nv.iter().map(|p| self.add_succ(*p));
    }
    pub fn clear_pred(&mut self) {
        self.pred.clear();
    }
    pub fn clear_succ(&mut self) {
        self.succ.clear();
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
