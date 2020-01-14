use crate::_tests::parser::helpers::{test_eval, assert_number_node};
use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;


#[test]
fn eval() {
    let node = PrintNode::new("hello");
    assert_eq!(node.string_param, "hello");
    assert_eq!(test_eval(&*node), "eval_print");
}

#[test]
fn construct_and_eval_with_param_nodes() {
    let mut node = PrintNode::new("");
    node.add_param(NumberNode::new(1234));
    node.add_param(NumberNode::new(5678));
    assert_eq!(node.params.len(), 2);
    assert_number_node(&node.params[0], 1234);
    assert_number_node(&node.params[1], 5678);

}