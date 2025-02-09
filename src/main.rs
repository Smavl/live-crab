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
    let mut cfg = ControlFlowGraph::from(&prog);
    cfg.fast_perform_liveness_analysis();

    let (live_in,live_out) = cfg.get_live_sets();

    println!("Program {prog}");
    println!("live_in: {:?}",live_in );
    println!("live_out: {:?}",live_out );

    for (idx, v) in live_in.iter().enumerate() {
        println!("{idx}, in: {:?}, out: {:?}",*v, live_out.get(idx).unwrap());
    }


    println!("{:?}", cfg.get_live_range(String::from("c")));

    Ok(())
}
