use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::_tests::parser::test_node_evaluator::test_eval;
use crate::parser::ifnode::IfNode;

#[test]
fn eval() {
    let left = NumberNode::new(1111);
    let right = NumberNode::new(2222);
    let then = PrintNode::new("impossible");
    let node = IfNode::new(left, right, '>', then);
    assert_eq!(test_eval(&node), "if 1111 > 2222 print impossible")
}