use live_crab::ast::*;
use live_crab::lexer::Lexer;
use live_crab::liveness::ControlFlowGraph;
use live_crab::liveness::NodeKind;
use live_crab::parser::Parser;

mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

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
        let got = cfg.get_nodes().first().unwrap().get_node_kind().clone();
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
        let cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = [make_ass_node_lit("a", 2),
            make_ass_node_lit("b", 3),
            make_return(Expr::Id(String::from("a")))];
        assert_eq!(
            Some(got.first().unwrap().get_node_kind()),
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
        let lexer = Lexer::new(file);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got = cfg.get_nodes();
        let want_vec = [
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
            Some(got.first().unwrap().get_node_kind()),
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
        let want_vec = [
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
            Some(got.first().unwrap().get_node_kind()),
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
        assert!(got1_use.is_empty());
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
    // if and return 
    #[test]
    fn use_if_cond() {
        let s = "a = 42; if ( a < 24 ) {a = 69;}";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got_def = cfg.get_node(1).get_defs();
        let got_use = cfg.get_node(1).get_uses();
        assert!(got_def.is_empty(), "{:?}", cfg.get_node(1));
        assert!(got_use.contains("a"), "{:?}", cfg.get_node(1));
    }
    #[test]
    fn use_return() {
        let s = "a = 42; return a;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let got_def = cfg.get_node(1).get_defs();
        let got_use = cfg.get_node(1).get_uses();
        assert!(got_def.is_empty(), "{:?}", cfg.get_node(1));
        assert!(got_use.contains("a"), "{:?}", cfg.get_node(1));
    }
    #[test]
    fn use_def_book_ex() {
        let s = get_str_from_path("examples/book_ex").unwrap();
        let lexer = Lexer::new(&s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        //This is how it should be:
        //0: a = 0;
        //	def: {"a"}, use: {}
        //1: b = a + 1;
        //	def: {"b"}, use: {"a"}
        //2: c = c + 1;
        //	def: {"c"}, use: {"c"}
        //3: a = b * 2;
        //	def: {"a"}, use: {"b"}
        //4: if a < 9
        //	def: {}, use: {"a"}
        //5: return c;
        //	def: {}, use: {"c"}

        fn check_def_use(cfg: &ControlFlowGraph, node_idx: usize, wanted_def: Vec<&str>, wanted_use: Vec<&str>) {
            for u in wanted_use.iter() {
               assert!(cfg.get_node(node_idx).contains_use(u.to_string()));
            }
            for d in wanted_def.iter() {
               assert!(cfg.get_node(node_idx).contains_def(d.to_string()));
            }
        }

        // Node 0:
        check_def_use(&cfg, 0, vec!["a"], vec![]);
        // Node 1:
        check_def_use(&cfg, 1, vec!["b"], vec!["a"]);
        // Node 2:
        check_def_use(&cfg, 2, vec!["c"], vec!["c"]);
        // Node 3:
        check_def_use(&cfg, 3, vec!["a"], vec!["b"]);
        // Node 4:
        check_def_use(&cfg, 4, vec![], vec!["a"]);
        // Node 5:
        check_def_use(&cfg, 5, vec![], vec!["c"]);
    }
    // succ and pred tests - positive
    // SUCC
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
        assert!(got3, "Node 3 did have a successor");
    }
    // TODO: Verify this test, i was going out of my mind
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
    #[test]
    fn succ_while() {
        let s = "a = 901;
        while ( a < 4 ) {
            a = a+2;
            a = a-1;
        } 
        return a;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        let cond_branch_true = cfg.get_node(1).get_succs().contains(&2);
        let cond_branch_false = cfg.get_node(1).get_succs().contains(&4);
        assert!(cond_branch_true, "Cond did not branch to body");
        assert!(cond_branch_false, "Cond did not branch to after the body");
    }
    #[test]
    fn succ_if() {
        let s = "i = 0;
        if ( a < 1337 ) {
            a = a+1;
            a = 2+1;
            a = a*2;
        }
        return a;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        // ------------------------------
        // node 0: assignment
        // node 1: cond (expr) - If
        // node 2: body (assignment)
        // node 3: body (assignment)
        // node 4: body (assignment)
        // node 5: return
        // ------------------------------
        let cond_is_after_initial = cfg.get_node(0).get_succs().contains(&1);
        let cond_branch_true = cfg.get_node(1).get_succs().contains(&2);
        let cond_branch_false = cfg.get_node(1).get_succs().contains(&5);
        let last_body_branch_to_rest = cfg.get_node(4).get_succs().contains(&5);
        let return_is_empty = cfg.get_node(5).get_succs().is_empty();
        assert!(
            cond_is_after_initial,
            "Node 0 did not continue to the condition (if)"
        );
        assert!(cond_branch_true, "Cond did not branch to body");
        assert!(cond_branch_false, "Cond did not branch to after the body");
        assert!(
            last_body_branch_to_rest,
            "Last node in body did not continue"
        );
        assert!(return_is_empty, "Last statment did have a successor");
    }

    // Pred test
    #[test]
    fn pred_if() {
        let s = "i = 0;
        if ( a < 1337 ) {
            a = a+1;
            a = 2+1;
            a = a*2;
        }
        return a;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        // ------------------------------
        // node 0: assignment
        // node 1: cond (expr) - If
        // node 2: body (assignment)
        // node 3: body (assignment)
        // node 4: body (assignment)
        // node 5: return
        // ------------------------------
        let no_pred_to_entry = cfg.get_node(0).get_preds().is_empty();
        let entry_to_cond = cfg.get_node(1).get_preds().contains(&0);
        let return_has_cond_preds = cfg.get_node(5).get_preds().contains(&1);
        let return_has_body_preds = cfg.get_node(5).get_preds().contains(&4);
        debug_nodes(cfg);
        assert!(no_pred_to_entry, "There was pred to the entry");
        assert!(entry_to_cond, "The entry was not a pred to the cond");
        assert!(
            return_has_cond_preds,
            "The false branch was not a pred to return"
        );
        assert!(
            return_has_body_preds,
            "The true branch (last node in the body) was not a pred to return"
        );
    }

    // TODO: Verify this test, i was going out of my mind
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
        debug_nodes(cfg);
        assert!(got1, "Node 0 was not empty");
        assert!(got2, "Node 0 did not continue to the body");
        assert!(got3, "Node 2 did not loop back to the body");
        assert!(got4, "Node 1 did not continue to the cond");
        assert!(got5, "Node 2 did not continue to the return");
    }
    #[test]
    fn pred_book_ex() {
        let s = get_str_from_path("examples/book_ex").unwrap();
        let lexer = Lexer::new(&s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        //This is how it should be:
        //0: a = 0;
        //	pred: {}
        //1: b = a + 1;
        //	pred: {0, 4}
        //2: c = c + 1;
        //	pred: {1}
        //3: a = b * 2;
        //	pred: {2}
        //4: if a < 9
        //	pred: {3}
        //5: return c;
        //	pred: {4}

        fn check_pred(cfg: &ControlFlowGraph, node_idx: usize, size: usize, wanted_vec: Vec<usize>) {
            assert_eq!(cfg.get_node(node_idx).get_preds().len(), size, "It was excepted to have size {size}");
            assert_eq!(wanted_vec.len(), size, "Dev error!");
            for wanted in wanted_vec {
                assert!(cfg.get_node(node_idx).get_preds().contains(&wanted), "Node {wanted} was not a pred to Node {node_idx}" );
            }
        }

        // Node 0:
        assert_eq!(cfg.get_node(0).get_preds().len(), 0, "Node 0 was not empty");
        check_pred(&cfg, 0, 0, vec![]);
        // Node 1:
        check_pred(&cfg, 1, 2, vec![0,4]);
        // Node 2:
        check_pred(&cfg, 2, 1, vec![1]);
        // Node 3:
        check_pred(&cfg, 3, 1, vec![2]);
        // Node 4:
        check_pred(&cfg, 4, 1, vec![3]);
        // Node 5:
        check_pred(&cfg, 5, 1, vec![4]);
    }
    #[test]
    fn succ_book_ex() {
        let s = get_str_from_path("examples/book_ex").unwrap();
        let lexer = Lexer::new(&s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let cfg = ControlFlowGraph::from(&prog);
        //This is how it should be:
        //0: a = 0;
        //	succ: {1}
        //1: b = a + 1;
        //	succ: {2}
        //2: c = c + 1;
        //	succ: {3}
        //3: a = b * 2;
        //	succ: {4}
        //4: if a < 9
        //	succ: {1, 5}
        //5: return c;
        //	succ: {}

        fn check_succ(cfg: &ControlFlowGraph, node_idx: usize, size: usize, wanted_vec: Vec<usize>) {
            assert_eq!(wanted_vec.len(), size, "Dev error!");
            assert_eq!(cfg.get_node(node_idx).get_succs().len(), size, "It was excepted to have size {size}");
            for wanted in wanted_vec {
                assert!(cfg.get_node(node_idx).get_succs().contains(&wanted), "Node {wanted} was not a succ to Node {node_idx}" );
            }
        }

        // Node 0:
        check_succ(&cfg, 0, 1, vec![1]);
        // Node 1:
        check_succ(&cfg, 1, 1, vec![2]);
        // Node 2:
        check_succ(&cfg, 2, 1, vec![3]);
        // Node 3:
        check_succ(&cfg, 3, 1, vec![4]);
        // Node 4:
        check_succ(&cfg, 4, 2, vec![1,5]);
        // Node 5:
        check_succ(&cfg, 5, 0, vec![]);
    }
    // in and out tests
    // in set
    #[test]
    fn in_set_contains_use() {
        let s = "b = 41;a = b + 1;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let mut cfg = ControlFlowGraph::from(&prog);
        cfg.fast_perform_liveness_analysis();
        let got_in = cfg.get_live_in(1);
        assert_eq!(got_in.len(), 1, "Second node did not have exactly 1 live in variable");
        assert!(got_in.contains(&String::from("b")), "Did not contain variable b")
    }
    // out set
    #[test]
    fn out_set_succ() {
        let s = "b = 41;a = b + 1;";
        let lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let mut cfg = ControlFlowGraph::from(&prog);
        cfg.fast_perform_liveness_analysis();
        let got_out = cfg.get_live_out(0);
        assert_eq!(got_out.len(), 1, "First node did not have exactly 1 live in variable");
        assert!(got_out.contains(&String::from("b")), "Did not contain variable b")
    }

    // liveness book_ex 
    #[test]
    fn liveranges_book_ex () {
        let s = get_str_from_path("examples/book_ex").unwrap();
        let lexer = Lexer::new(&s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let prog = parser.parse();
        let mut cfg = ControlFlowGraph::from(&prog);
        cfg.fast_perform_liveness_analysis();

        let c_live = cfg.get_live_range(String::from("c"));
        let b_live = cfg.get_live_range(String::from("b"));
        let a_live = cfg.get_live_range(String::from("a"));

        assert!(c_live.contains(&(0,1)));
        assert!(c_live.contains(&(1,2)));
        assert!(c_live.contains(&(2,3)));
        assert!(c_live.contains(&(3,4)));
        assert!(c_live.contains(&(4,5)));
        assert!(c_live.contains(&(4,1)));
        assert_eq!(c_live.len(), 6);

        assert!(b_live.contains(&(1,2)));
        assert!(b_live.contains(&(2,3)));
        assert_eq!(b_live.len(), 2);

        assert!(a_live.contains(&(0,1)));
        assert!(a_live.contains(&(3,4)));
        assert!(a_live.contains(&(4,1)));
        assert!(!a_live.contains(&(1,2)));
        assert!(!a_live.contains(&(2,3)));
        assert_eq!(a_live.len(), 3);
    }

}
