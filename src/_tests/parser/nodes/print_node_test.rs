use crate::_tests::parser::helpers::*;
use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::string_node::StringNode;
use crate::parser::node::Node;


#[test]
fn eval() {
    let node = PrintNode::with_string_param("hello");
    assert_eq!(node.params.len(), 1);
    assert_string_node(&node.params[0], "hello");
    assert_eq!(test_eval(&*node), "eval_print");
}

#[test]
fn construct_and_eval_with_param_nodes() {
    let mut node = PrintNode::new(1111);
    node.add_param(NumberNode::new(1234));
    node.add_param(StringNode::new("abcd"));
    assert_eq!(node.params.len(), 2);
    assert_number_node(&node.params[0], 1234);
    assert_string_node(&node.params[1], "abcd");
    assert_eq!(node.line(), 1111);
}