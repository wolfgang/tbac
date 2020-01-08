use crate::_tests::parser::test_node_evaluator::test_eval;
use crate::parser::print_node::PrintNode;


#[test]
fn eval() {
    let node = PrintNode::new("hello");
    assert_eq!(test_eval(&node), "print hello");
}