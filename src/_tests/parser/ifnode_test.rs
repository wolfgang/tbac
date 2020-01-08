use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::_tests::parser::test_node_evaluator::TestNodeEvaluator;
use crate::parser::ifnode::IfNode;
use crate::parser::node::Node;

#[test]
fn eval() {
    let left = NumberNode::new(1111);
    let right = NumberNode::new(2222);
    let then = PrintNode::new("impossible");
    let node = IfNode::new(left, right, '>', then);
    let mut evaluator = TestNodeEvaluator::new();
    let code = node.eval(&mut evaluator);
    assert_eq!(code, "if 1111 > 2222 print impossible")
}