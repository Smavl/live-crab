use crate::ast::*;
use std::{collections::HashSet, fmt::format};

#[derive(Debug, PartialEq, Eq)]
pub struct ControlFlowGraph {
    nodes: Vec<Node>,
    live_in: Vec<HashSet<String>>,
    live_out: Vec<HashSet<String>>,
    live_ranges_found: bool,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        ControlFlowGraph { 
            nodes: Vec::new(), 
            live_in: Vec::new(),
            live_out: Vec::new(),
            live_ranges_found: false,
        }
    }

    pub fn from(p: &Program) -> Self {
        let mut cfg = ControlFlowGraph::new();
        cfg.nodes = flatten_program(p);
        cfg
    }

    pub fn get_nodes(&self) -> Vec<&Node> {
        self.nodes.iter().collect()
    }
    pub fn get_node(&self, n: usize) -> &Node {
        if let Some(n) = self.nodes.get(n) {
            n
        } else {
            panic!("That node could not be found")
        }
    }

    // in[n] = use[n] U (out[n] - def[n])
    fn calc_in(&self, node: &Node) -> HashSet<String> {
        let out_diff_def = self
            .calc_out(node)
            .difference(node.get_defs())
            .cloned()
            .collect();

        node.get_uses().union(&out_diff_def).cloned().collect()
    }

    // out[n] = U in[s], where s = succ[n]
    fn calc_out(&self, node: &Node) -> HashSet<String> {
        let succs = node.get_succs();
        let mut outs = HashSet::new();

        for s in succs {
            outs.extend(self.get_live_in(*s).iter().cloned());
        }
        outs

    }

    pub fn perform_liveness_analysis(&mut self) {
        // init
        for _ in 0..self.nodes.len() {
            self.live_in.push(HashSet::new());
            self.live_out.push(HashSet::new());
        }

        let mut og_in = None;
        let mut og_out = None;

        let mut it= 0;

        loop {
            for (idx, n) in self.nodes.iter().enumerate() {
                self.live_in[idx] = self.calc_in(n);
                self.live_out[idx] = self.calc_out(n);
            }
            it += 1;
            if og_out == Some(self.live_out.clone()) && og_in == Some(self.live_in.clone()) {
                break;
            }
            og_in = Some(self.live_in.clone());
            og_out = Some(self.live_out.clone());
        }
        println!("Iterations: {it}");
        self.live_ranges_found = true;
    }

    pub fn fast_perform_liveness_analysis(&mut self) {
        // init
        for _ in 0..self.nodes.len() {
            self.live_in.push(HashSet::new());
            self.live_out.push(HashSet::new());
        }

        let mut og_in = None;
        let mut og_out = None;

        let mut it= 0;

        loop {
            for i in (0..self.nodes.len()).rev() {
                self.live_in[i] = self.calc_in(self.get_node(i));
                self.live_out[i] = self.calc_out(self.get_node(i));
            }
            //for (idx, n) in self.nodes.iter().enumerate() {
            //    self.live_in[idx] = self.calc_in(n);
            //    self.live_out[idx] = self.calc_out(n);
            //}
            it += 1;
            if og_out == Some(self.live_out.clone()) && og_in == Some(self.live_in.clone()) {
                break;
            }
            og_in = Some(self.live_in.clone());
            og_out = Some(self.live_out.clone());
        }
        println!("Iterations: {it}");
        self.live_ranges_found = true;
    }

    pub fn get_live_in(&self,idx:usize) -> &HashSet<String> {
        self.live_in.get(idx).unwrap()
    }

    pub fn get_live_out(&self,idx:usize) -> &HashSet<String> {
        self.live_out.get(idx).unwrap()
    }

    pub fn get_live_sets(&self) -> (&Vec<HashSet<String>>,&Vec<HashSet<String>>) {
        (&self.live_in, &self.live_out)
    }

    pub fn get_live_range(&self, var: String) -> Vec<(usize,usize)> {
        println!("Live range for {var}");
        let mut res = Vec::new();
        for (idx, n) in self.nodes.iter().enumerate() {
            let succ = n.get_succs();
            for s in succ.iter() {
                if self.live_in.get(*s).expect("").contains(&var) {
                    res.push((idx,*s));
                }
            }
        }
        res
    }

    pub fn generate_dot(&self) -> String {
        let mut sb = String::from(
            "digraph CFG {\n\tnode [shape=rectangle];\n\n");

        let mut blocks = String::new();
        let mut edges= String::new();

        for node in self.nodes.iter() {
            let exp = format!("{}",node.node_kind);
            let idx = node.idx;
            let s = format!("\tblock{idx} [label=\"{exp}\"];\n");
            blocks.push_str(s.as_str());

            for s in node.get_succs() {
                //if self.live_out.len() < node.idx {continue;}
                //if self.live_in.len() < *s {continue;}
                let mut live = String::from("");
                if self.live_ranges_found {
                    live = self.live_out[node.idx]
                        .intersection(&self.live_in[*s])
                        .map(|l| l.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                } 

                edges.push_str(format!("\tblock{idx} -> block{s} [label=\"{live}\"];\n").as_str());
            }

        }
        sb.push_str(&blocks);
        sb.push('\n');

        sb.push_str(&edges);

        sb.push('}');

        sb
    }

}

// WARN: what is this??
impl Default for ControlFlowGraph {
    fn default() -> Self {
        Self::new()
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
        // If the the node has no succs and is not a return statements
        if node.get_succs().is_empty() {
            if let NodeKind::Return(_) = node.get_node_kind() {
            } else {
                // add the next node ass a succ
                node.add_succ(self.get_offset() + 1);
            }
        }

        for idx in node.get_succs() {
            if self.nodes.get(*idx).is_some() {
                self.nodes[*idx].add_pred(cur_off);
            }
        }

        // If the node is a succ to any node,
        // and add it to the node's pred set
        for n in self.nodes.iter() {
            if n.get_succs().contains(&cur_off) {
                node.add_pred(n.get_node_idx());
            }
        }

        // TODO: Extract this to a "handle_use()"-function
        if let NodeKind::Condition(e) = node.get_node_kind() {
            node.use_extend(e.clone().iter().collect::<HashSet<String>>());
        }
        if let NodeKind::Return(e) = node.get_node_kind() {
            node.use_extend(e.clone().iter().collect::<HashSet<String>>());
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
                let node = handle_assignment(state.get_offset(),a);

                state.add_node(node);
            }
            r @ Statement::Return(_) => {
                let node = handle_return(state.get_offset(), r);
                state.add_node(node);
            }
            Statement::If(cond, body) => {
                let body_start = state.get_offset() + 1;

                let mut cond_node = Node::new(state.get_offset(),NodeKind::Condition(cond.clone()));

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

                let mut cond_node = Node::new(body_end,NodeKind::Condition(cond.clone()));

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
                let mut cond_node = Node::new(state.get_offset(), NodeKind::Condition(cond.clone()));
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
fn handle_assignment(idx: usize,stmt: &Statement) -> Node {
    match stmt {
        Statement::Assignment(lvl, e) => {
            let lval = lvl.clone();
            let exp = e.clone();

            let mut node = Node::new(idx,NodeKind::from((lval, exp)));

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
fn handle_return(idx: usize, stmt: &Statement) -> Node {
    match stmt {
        Statement::Return(e) => {
            Node::new(idx,NodeKind::from(e.clone()))
        }
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
    idx : usize,
    node_kind: NodeKind,
    use_set: HashSet<String>,
    def_set: HashSet<String>,
    in_set: HashSet<String>,
    pred: HashSet<usize>,
    succ: HashSet<usize>,
}

impl Node {
    pub fn new(idx: usize, node_kind: NodeKind) -> Self {
        Node {
            idx,
            node_kind,
            // According to the compiler book, the sets below
            // should stored in the CFG to improve modulatity
            use_set: HashSet::new(),
            def_set: HashSet::new(),
            pred: HashSet::new(),
            succ: HashSet::new(),
            in_set: HashSet::new(),
        }
    }

    pub fn get_node_idx(&self) -> usize {
        self.idx
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

    pub fn contains_def(&self, d:String) -> bool {
        self.def_set.contains(&d)
    }
    pub fn contains_use(&self, d:String) -> bool {
        self.use_set.contains(&d)
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
