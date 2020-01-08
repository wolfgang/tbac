use crate::_tests::parser::test_node_evaluator::TestNodeEvaluator;
use crate::parser::node::Node;
use crate::parser::print_node::PrintNode;


#[test]
fn eval() {
    let mut evaluator = TestNodeEvaluator::new();
    let node = PrintNode::new("hello");
    let code = node.eval(&mut evaluator);

    assert_eq!(code, "print hello");
}