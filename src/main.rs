use live_crab::ast::*;
use live_crab::lexer::Lexer;
// use live_crab::lexer::Token;
use live_crab::liveness::*;
use live_crab::parser::Parser;
use std::{env, io};

fn create_binop_rlit(left: Expr, op: Operator, right: i32) -> Expr {
    Expr::BinOp(Box::new(left), op, Box::new(Expr::Int(right)))
}
fn create_binop_lit(left: i32, op: Operator, right: i32) -> Expr {
    Expr::BinOp(Box::new(Expr::Int(left)), op, Box::new(Expr::Int(right)))
}
fn get_str_from_path(path: &str) -> Option<String> {
    let cwd = env::current_dir().unwrap();
    println!("Current working directory: {}", cwd.display());
    std::fs::read_to_string(path).ok()
}

fn main() -> io::Result<()> {
    let s = "a = 41;do {a = a+1;} while( a < 42 ); return a;";
    let lexer = Lexer::new(s);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();
    let cfg = ControlFlowGraph::from(&prog);
    let got1 = cfg.get_node(0).get_succs().contains(&1);
    let got2 = cfg.get_node(1).get_succs().contains(&2);
    let got3 = cfg.get_node(2).get_succs().contains(&1);
    let got4 = cfg.get_node(2).get_succs().contains(&3);
    let got5 = cfg.get_node(3).get_succs().is_empty();
    for (idx, n) in cfg.get_nodes().iter().enumerate() {
        println!(
            "Node {idx}: {:?} \n\tpreds: {:?}, succ: {:?}",
            n.get_node_kind(),
            n.get_preds(),
            n.get_succs()
        );
    }
    assert!(got1, "Node 1 did not succede to the body");
    assert!(got2, "Node 2 did not succede to the cond");
    assert!(got3, "Node 3 (cond) not branch conditionally, true");
    assert!(got4, "Node 3 (cond) not branch conditionally, false ");
    assert!(got5, "Node 4 did have a successor");

    Ok(())
}
