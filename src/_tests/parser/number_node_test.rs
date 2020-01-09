use crate::_tests::parser::test_node_evaluator::test_eval;
use crate::parser::number_node::NumberNode;

#[test]
fn eval() {
    let node = NumberNode::new(1234);
    assert_eq!(node.value, 1234);
    assert_eq!(test_eval(&*node), "eval_number");
}
