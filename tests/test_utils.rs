use live_crab::{ast::{Expr, Operator}, liveness::{ControlFlowGraph, NodeKind}};

pub fn get_str_from_path(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
pub fn make_ass_node_lit(id: &str, lit: i32) -> NodeKind {
    NodeKind::Assignment(Box::new(Expr::Id(id.to_string())), Box::new(Expr::Int(lit)))
}
pub fn make_ass_node_exp(id: &str, exp: Expr) -> NodeKind {
    NodeKind::Assignment(Box::new(Expr::Id(id.to_string())), Box::new(exp))
}
pub fn create_binop_lit(left: i32, op: Operator, right: i32) -> Expr {
    Expr::BinOp(Box::new(Expr::Int(left)), op, Box::new(Expr::Int(right)))
}

pub fn create_binop_rlit(left: Expr, op: Operator, right: i32) -> Expr {
    Expr::BinOp(Box::new(left), op, Box::new(Expr::Int(right)))
}
pub fn make_inc(id: &str) -> Expr {
    Expr::BinOp(
        Box::new(Expr::Id(id.to_string())),
        Operator::Plus,
        Box::new(Expr::Int(1)),
    )
}

pub fn make_return(e: Expr) -> NodeKind {
    NodeKind::Return(Box::new(e))
}
pub fn debug_nodes(cfg: ControlFlowGraph) {
    for (idx, n) in cfg.get_nodes().iter().enumerate() {
        println!("Node {idx}: {:?}\n", n)
    }
}

