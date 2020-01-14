use crate::parser::number_node::NumberNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::_tests::parser::fake_node_evaluator::test_eval;
use crate::_tests::helpers::*;

#[test]
fn construct_and_eval() {
    let mut node = SequenceNode::new();
    node.add(NumberNode::new(1111));
    node.add(PrintNode::new("hello"));

    assert_eq!(node.children.len(), 2);
    assert_number_node(&node.children[0], 1111);
    assert_print_node(&node.children[1], "hello");

    assert_eq!(test_eval(&node), "eval_numbereval_print");
}
