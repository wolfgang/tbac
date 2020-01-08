use crate::_tests::parser::test_node_evaluator::TestNodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::node::Node;

#[test]
fn eval() {
    let mut evaluator = TestNodeEvaluator::new();
    let node = NumberNode::new(1234);
    let code = node.eval(&mut evaluator);
    assert_eq!(code, "1234");
}