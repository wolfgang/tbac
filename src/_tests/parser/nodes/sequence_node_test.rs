use crate::parser::number_node::NumberNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::_tests::parser::helpers::*;

#[test]
fn construct_and_eval() {
    let mut node = SequenceNode::new();
    node.add(NumberNode::new(1111));
    node.add(PrintNode::with_string_param("hello"));

    assert_eq!(node.children.len(), 2);
    assert_number_node(&node.children[0], 1111);
    assert_print_node(&node.children[1], "hello");

    assert_eq!(test_eval(&node), "eval_sequence");
}
