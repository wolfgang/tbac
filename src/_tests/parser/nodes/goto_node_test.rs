use crate::parser::number_node::NumberNode;
use crate::_tests::parser::helpers::{assert_number_node, test_eval};
use crate::parser::goto_node::GotoNode;
use crate::parser::node::Node;

#[test]
fn construct_and_eval() {
    let node = GotoNode::new(NumberNode::new(10), 20);
    assert_number_node(&node.destination, 10);
    assert_eq!(node.line(), 20);
    assert_eq!(test_eval(&*node), "eval_goto");
}