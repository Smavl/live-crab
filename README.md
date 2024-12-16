# Description

This a small education project exploring liveness analysis. 

The codebase has a "self contained" lexer and parser.

There is no plan for this language to compile into actual bits and bytes. It is merely a tool to explore techniques and algorithms introduced in my compiler course.



# Language

The language is quite simple:

A program, consists of one or more statements

```rust
pub struct Program {
    pub stmts: Vec<Statement>,
}
```

A Statement is one of following variants:

```rust
pub enum Statement {
    Assignment(Box<Expr>, Box<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Vec<Statement>),
    While(Box<Expr>, Vec<Statement>),
    DoWhile(Vec<Statement>, Box<Expr>),
}
```

As of now, the parse is the dictator. If the parser is happy, it is a valid program.

# Roadmap


Want to:

- [ ] Examples in the README
- [ ] Finish liveness analyser
- [ ] fix up tests (clean, unit test, less examples)
- [ ] General code cleaning
- [ ] Reduce `clone()`'s. Plausible solutions:
	- [ ] Have `Expr` implement the `Copy` trait (4Head)
	- [ ] Use `str` instead of `String` (Figure out lifetime and their specifiers)
- [ ] Minor semantic analysis (with minor error recovery)
- [ ] CLI compatibility (Input an actual file)
- [ ] Visualize with dot
- [ ] Fire up local webserver, with a built-in editor(default and vim.js). Show output based in editor input.
	- [ ] Parse it to js? WebAssembly?

Might also want to:
- [ ] Register allocation, k-coloring
	- [ ] With coalescing (11.4)
- [ ] Optimazations

