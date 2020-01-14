use crate::parser::ifnode::IfNode;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::_tests::parser::helpers::*;

#[test]
fn eval() {
    let node = IfNode::new(
        NumberNode::new(1111),
        NumberNode::new(2222),
        '>',
        PrintNode::new("impossible"));

    assert_number_node(&node.left, 1111);
    assert_number_node(&node.right, 2222);
    assert_eq!(node.relop, '>');
    assert_print_node(&node.then, "impossible");

    assert_eq!(test_eval(&*node), "eval_if")
}
