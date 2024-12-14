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
fn make_ass_node_lit(id: &str, lit: i32) -> NodeKind {
    NodeKind::Assignment(Box::new(Expr::Id(id.to_string())), Box::new(Expr::Int(lit)))
}
fn make_ass_node_exp(id: &str, exp: Expr) -> NodeKind {
    NodeKind::Assignment(Box::new(Expr::Id(id.to_string())), Box::new(exp))
}
fn make_inc(id: &str) -> Expr {
    Expr::BinOp(
        Box::new(Expr::Id(id.to_string())),
        Operator::Plus,
        Box::new(Expr::Int(1)),
    )
}

fn make_return(e: Expr) -> NodeKind {
    NodeKind::Return(Box::new(e))
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
        let want = make_ass_node_lit("a", 42);
        let got = cfg.get_nodes().get(0).unwrap().get_node_kind().clone();
        assert_eq!(Some(got), Some(want));
    }
    #[test]
    fn flatten_simple_example1() {
        // a = 2; b = 3; return a;
        let file = get_str_from_path("examples/s1").unwrap();
        let lexer = Lexer::new(file.as_str());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let mut cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = vec![
            make_ass_node_lit("a", 2),
            make_ass_node_lit("b", 3),
            make_return(Expr::Id(String::from("a"))),
        ];
        assert_eq!(
            Some(got.get(0).unwrap().get_node_kind()),
            Some(&want_vec[0])
        );
        assert_eq!(
            Some(got.get(1).unwrap().get_node_kind()),
            Some(&want_vec[1])
        );
        assert_eq!(
            Some(got.get(2).unwrap().get_node_kind()),
            Some(&want_vec[2])
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
        let mut cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = vec![
            make_ass_node_lit("i", 0),
            NodeKind::Condition(Box::new(Expr::BinOp(
                Box::new(Expr::Id("i".to_string())),
                Operator::LessThan,
                Box::new(Expr::Int(3)),
            ))),
            make_ass_node_exp("i", make_inc("i")),
            NodeKind::Assignment(
                Box::new(Expr::Id("i".to_string())),
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::Plus,
                    Box::new(Expr::Id("i".to_string())),
                )),
            ),
            NodeKind::Assignment(
                Box::new(Expr::Id("i".to_string())),
                Box::new(Expr::BinOp(
                    Box::new(Expr::Id("i".to_string())),
                    Operator::Mult,
                    Box::new(Expr::Id("i".to_string())),
                )),
            ),
            NodeKind::Return(Box::new(Expr::Id("a".to_string()))),
        ];
        assert_eq!(got.len(), 5);
        assert_eq!(
            Some(got.get(0).unwrap().get_node_kind()),
            Some(&want_vec[0])
        );
        assert_eq!(
            Some(got.get(1).unwrap().get_node_kind()),
            Some(&want_vec[1])
        );
        assert_eq!(
            Some(got.get(2).unwrap().get_node_kind()),
            Some(&want_vec[2])
        );
        assert_eq!(
            Some(got.get(3).unwrap().get_node_kind()),
            Some(&want_vec[3])
        );
        assert_eq!(
            Some(got.get(4).unwrap().get_node_kind()),
            Some(&want_vec[4])
        );
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
        let cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = vec![
            make_ass_node_lit("i", 0),
            make_ass_node_exp("i", make_inc("i")),
            make_ass_node_exp("i", make_inc("i")),
            NodeKind::Condition(Box::new(Expr::BinOp(
                Box::new(Expr::Id("i".to_string())),
                Operator::LessThan,
                Box::new(Expr::Int(9)),
            ))),
            make_return(Expr::Id("i".to_string())),
        ];
        assert_eq!(
            Some(got.get(0).unwrap().get_node_kind()),
            Some(&want_vec[0])
        );
        assert_eq!(
            Some(got.get(1).unwrap().get_node_kind()),
            Some(&want_vec[1])
        );
        assert_eq!(
            Some(got.get(2).unwrap().get_node_kind()),
            Some(&want_vec[2])
        );
        assert_eq!(
            Some(got.get(3).unwrap().get_node_kind()),
            Some(&want_vec[3])
        );
        assert_eq!(
            Some(got.get(4).unwrap().get_node_kind()),
            Some(&want_vec[4])
        );
    }

    // Use and Def tests
    #[test]
    fn node_single_def_and_empty_use() {
        let s = "a = 42;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got1_def = cfg.get_node(0).get_defs();
        let got1_use = cfg.get_node(0).get_uses();
        assert!(got1_def.contains("a"));
        assert!(got1_use.is_empty())
    }
    #[test]
    fn node_two_def_and_use() {
        let s = "b = 41;a = b + 1;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got_def = cfg.get_node(1).get_defs();
        let got_use = cfg.get_node(1).get_uses();
        assert!(got_def.contains("a"));
        assert!(got_use.contains("b"));
    }
    // succ and pred tests
    #[test]
    fn succ_three_ass() {
        let s = "a = 41;
             b = 1;
             x = a;
             ";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got1 = cfg.get_node(0).get_succs().contains(&1);
        let got2 = cfg.get_node(1).get_succs().contains(&2);
        let got3 = cfg.get_node(2).get_succs().is_empty();
        assert!(got1, "Node 1 did not succede Node 0");
        assert!(got2, "Node 2 did not succede Node 1");
        assert!(got3, "Node 3 did succede have a successor");
    }
    // !TODO Verify this test, i was going out of my mind
    #[test]
    fn succ_do_while() {
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
                "Node {idx}: succ: {:?}, preds: {:?}",
                n.get_succs(),
                n.get_preds()
            );
        }
        assert!(got1, "Node 1 did not succede to the body");
        assert!(got2, "Node 2 did not succede to the cond");
        assert!(got3, "Node 3 (cond) not branch conditionally, true");
        assert!(got4, "Node 3 (cond) not branch conditionally, false ");
        assert!(got5, "Node 4 did have a successor");
    }
    // !TODO Verify this test, i was going out of my mind
    #[test]
    fn pred_do_while() {
        let s = "a = 41;do {a = a+1;} while( a < 42 ); return a;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        // node 0: assignment
        // node 1: body (assignment)
        // node 2: cond (expr)
        // node 3: return
        let got1 = cfg.get_node(0).get_preds().is_empty();
        let got2 = cfg.get_node(1).get_preds().contains(&0);
        let got3 = cfg.get_node(1).get_preds().contains(&2);
        let got4 = cfg.get_node(2).get_preds().contains(&1);
        let got5 = cfg.get_node(3).get_preds().contains(&2);
        for (idx, n) in cfg.get_nodes().iter().enumerate() {
            println!("Node {idx}: {:?}\n", n)
        }
        assert!(got1, "Node 0 was not empty");
        assert!(got2, "Node 0 did not continue to the body");
        assert!(got3, "Node 2 did not loop back to the body");
        assert!(got4, "Node 1 did not continue to the cond");
        assert!(got5, "Node 2 did not continue to the return");
    }
    // in and out tests
}
