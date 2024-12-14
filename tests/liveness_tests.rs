use live_crab::ast::*;
use live_crab::lexer::Lexer;
use live_crab::lexer::Token;
use live_crab::liveness::ControlFlowGraph;
use live_crab::liveness::{Node, NodeKind};
use live_crab::parser::Parser;

fn get_str_from_path(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

// helper function to wrap statements in a Node
// fn wrap_in_node(s: Statement) -> Node {
//     Node::new(s)
// }
//
// helper function to wrap expressions in a Node
fn make_ass_node_lit(id: &str, lit: i32) -> Node {
    Node::new(NodeKind::Assignment(
        Box::new(Expr::Id(id.to_string())),
        Box::new(Expr::Int(lit)),
    ))
}
fn make_ass_node_exp(id: &str, exp: Expr) -> Node {
    Node::new(NodeKind::Assignment(
        Box::new(Expr::Id(id.to_string())),
        Box::new(exp),
    ))
}
fn make_inc(id: &str) -> Expr {
    Expr::BinOp(
        Box::new(Expr::Id(id.to_string())),
        Operator::Plus,
        Box::new(Expr::Int(1)),
    )
}

fn make_return(e: Expr) -> Node {
    Node::new(NodeKind::Return(Box::new(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // flatten tests
    #[test]
    fn flatten_single_assignment() {
        let s = "a = 42;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        assert_eq!(cfg.get_nodes().len(), 1);
        // let ndk = NodeKind::Assignment(
        //     Box::new(Expr::Id(String::from("a"))),
        //     Box::new(Expr::Int(42)),
        // );
        // let want = Node::new(ndk);
        let want = make_ass_node_lit("a", 42);
        assert_eq!(cfg.get_nodes().get(0), Some(&want));
    }
    #[test]
    fn flatten_simple_example1() {
        // a = 2; b = 3; return a;
        let file = get_str_from_path("examples/s1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let got = ControlFlowGraph::from(&prog);
        let want_vec = vec![
            Node::new(NodeKind::Assignment(
                Box::new(Expr::Id("a".to_string())),
                Box::new(Expr::Int(2)),
            )),
            Node::new(NodeKind::Assignment(
                Box::new(Expr::Id("b".to_string())),
                Box::new(Expr::Int(3)),
            )),
            Node::new(NodeKind::Return(Box::new(Expr::Id("a".to_string())))),
        ];
        assert_eq!(
            got.get_nodes().get(0),
            Some(&want_vec[0]),
            "Got: {:?}\n\n",
            got
        );
        assert_eq!(
            got.get_nodes().get(1),
            Some(&want_vec[1]),
            "Got: {:?}\n\n",
            got
        );
        assert_eq!(
            got.get_nodes().get(2),
            Some(&want_vec[2]),
            "Got: {:?}\n\n",
            got
        );
    }
    #[test]
    fn flatten_if() {
        // Code from Examples/loop1 :
        // i = 0;
        // if ( i < 3 ) {
        //  i = i + 1;
        //  i = i + i;
        //  i = i * i;
        // }
        let file = "
         i = 0;
         if ( i < 3 ) {
          i = i + 1;
          i = i + i;
          i = i * i;
         }";
        // let file = get_str_from_path("examples/do_while").unwrap();
        let lexer = Lexer::new(file);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = vec![
            make_ass_node_lit("i", 0),
            Node::new(NodeKind::Condition(Box::new(Expr::BinOp(
                Box::new(Expr::Id("i".to_string())),
                Operator::LessThan,
                Box::new(Expr::Int(3)),
            )))),
            make_ass_node_exp("i", make_inc("i")),
            Node::new(NodeKind::Assignment(
                Box::new(Expr::Id("i".to_string())),
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::Plus,
                    Box::new(Expr::Id("i".to_string())),
                )),
            )),
            Node::new(NodeKind::Assignment(
                Box::new(Expr::Id("i".to_string())),
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::Mult,
                    Box::new(Expr::Id("i".to_string())),
                )),
            )),
            Node::new(NodeKind::Return(Box::new(Expr::Id("a".to_string())))),
        ];
        assert_eq!(got.len(), 5);
        assert_eq!(got.get(0), Some(&want_vec[0]));
        assert_eq!(got.get(1), Some(&want_vec[1]));
        assert_eq!(got.get(2), Some(&want_vec[2]));
        assert_eq!(got.get(3), Some(&want_vec[3]));
        assert_eq!(got.get(4), Some(&want_vec[4]));
    }
    #[test]
    fn flatten_do_while() {
        // Code from Examples/do_while :
        // i = 0;
        // do {
        // i = i + 1;
        // i = i + 1;
        // }
        // while (i < 9);
        // return i;
        let file = get_str_from_path("examples/do_while").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let got = ControlFlowGraph::from(&prog);
        let want_vec = vec![
            make_ass_node_lit("i", 0),
            make_ass_node_exp("i", make_inc("i")),
            make_ass_node_exp("i", make_inc("i")),
            Node::new(NodeKind::Condition(Box::new(Expr::BinOp(
                Box::new(Expr::Id("i".to_string())),
                Operator::LessThan,
                Box::new(Expr::Int(9)),
            )))),
            make_return(Expr::Id("i".to_string())),
        ];
        println!("got: {}", got);
        assert_eq!(
            got.get_nodes().get(0),
            Some(&want_vec[0]),
            "Got: {:?}, want {:?}",
            got,
            want_vec
        );
        assert_eq!(
            got.get_nodes().get(1),
            Some(&want_vec[1]),
            "Got: {:?}, want {:?}",
            got,
            want_vec
        );
        assert_eq!(
            got.get_nodes().get(2),
            Some(&want_vec[2]),
            "Got: {:?}, want {:?}",
            got,
            want_vec
        );
        assert_eq!(
            got.get_nodes().get(3),
            Some(&want_vec[3]),
            "Got: {:?}, want {:?}",
            got,
            want_vec
        );
        assert_eq!(
            got.get_nodes().get(4),
            Some(&want_vec[4]),
            "Got: {:?}, want {:?}",
            got,
            want_vec
        );

        //
    }
}