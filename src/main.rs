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
    let file = get_str_from_path("examples/do_while").unwrap();
    let lexer = Lexer::new(file.as_str());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();
    let cfg = ControlFlowGraph::from(&prog);
    println!("CFG:\n{}", cfg);
    Ok(())
}
