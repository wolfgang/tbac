use crate::_tests::parser::helpers::*;
use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::string_node::StringNode;


#[test]
fn eval() {
    let node = PrintNode::new("hello");
    assert_eq!(node.params.len(), 1);
    assert_string_node(&node.params[0], "hello");
    assert_eq!(test_eval(&*node), "eval_print");
}

#[test]
fn construct_and_eval_with_param_nodes() {
    let mut node = PrintNode::new("");
    node.add_param(NumberNode::new(1234));
    node.add_param(StringNode::new("abcd"));
    assert_eq!(node.params.len(), 3);
    assert_number_node(&node.params[1], 1234);
    assert_string_node(&node.params[2], "abcd");

}