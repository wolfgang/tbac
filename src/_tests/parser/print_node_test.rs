use crate::_tests::parser::test_node_evaluator::TestNodeEvaluator;
use crate::parser::node::Node;
use crate::parser::print_node::PrintNode;


#[test]
fn print_node_eval() {
    let mut evaluator = TestNodeEvaluator::new();
    let node = PrintNode::new("hello");
    node.eval(&mut evaluator);

    evaluator.assert_code(vec!["print hello"]);
}