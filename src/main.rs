use live_crab::lexer::Lexer;
use live_crab::liveness::*;
use live_crab::parser::Parser;
use std::io;

fn main() -> io::Result<()> {
    let s = "a = 42;";
    let lexer = Lexer::new(s);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();
    let cfg = ControlFlowGraph::from(&prog);
    let got1_def = cfg.get_node(0).get_defs();
    let got1_use = cfg.get_node(0).get_uses();
    assert!(got1_def.contains("a"));
    assert!(got1_use.is_empty());
    println!("{:?}", got1_use);
    println!("{:?}", got1_def);

    Ok(())
}
