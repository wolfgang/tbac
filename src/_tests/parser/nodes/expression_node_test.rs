use crate::_tests::parser::helpers::{assert_number_node, test_eval};
use crate::parser::expression_node::ExpressionNode;
use crate::parser::number_node::NumberNode;

#[test]
fn construct_and_eval() {
    let node = ExpressionNode::new(
        '+',
        NumberNode::new(1234),
        NumberNode::new(5678));

    assert_eq!(node.op, '+');
    assert_number_node(&node.left, 1234);
    assert_number_node(&node.right, 5678);

    assert_eq!(test_eval(&*node), "eval_expression");
}