use live_crab::lexer::Lexer; // Import the `Lexer` struct from `lexer.rs`
use live_crab::lexer::Token; // Import the `Token` enum (if it's in `lib.rs` or `lexer.rs`)
use std::{
    env,
    io::{self, BufReader, Read},
};
fn get_str_from_path(path: &str) -> Option<String> {
    let cwd = env::current_dir().unwrap();
    println!("Current working directory: {}", cwd.display());
    std::fs::read_to_string(path).ok()
}

fn main() -> io::Result<()> {
    // open example file
    // let path = "./../examples/s1";
    // let f = File::open(path)?;
    // let mut r = BufReader::new(f);
    // let mut buf = String::new();
    // r.read_to_string(&mut buf)?;

    // println!("----- Input string: -----\n{buf}");

    // println!("----- Lexer init : -----");

    // let lexer = Lexer::new(";");

    // let tks = lexer.tokenize();
    // println!("Tokens: {:?}", tks);

    let s = "a";
    let lexer = Lexer::new(s);
    let got = &lexer.tokenize();
    let mut want = Vec::new();
    want.push(Token::Id(String::from("a")));
    assert_eq!(got, &want);

    let s = get_str_from_path("../examples/s1");
    println!("Example: {:?}", s.unwrap());

    Ok(())
}
