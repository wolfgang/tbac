use crate::_tests::parser::test_node_evaluator::TestNodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::node::Node;

#[test]
fn eval() {
    let mut evaluator = TestNodeEvaluator::new();
    let node = NumberNode::new(1234);
    node.eval(&mut evaluator);
    evaluator.assert_code(vec!["1234"]);
}