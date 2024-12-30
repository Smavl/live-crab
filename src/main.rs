use live_crab::lexer::Lexer;
use live_crab::liveness::*;
use live_crab::parser::Parser;
use std::io;

pub fn get_str_from_path(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn main() -> io::Result<()> {
    let s = get_str_from_path("examples/book_ex").unwrap();
    let lexer = Lexer::new(&s);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();
    let cfg = ControlFlowGraph::from(&prog);
    println!("Cfg:\n{}",cfg);

    Ok(())
}
