use crate::_tests::parser::test_node_evaluator::test_eval;
use crate::parser::ifnode::IfNode;
use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;

#[test]
fn eval() {
    let node = IfNode::new(
        NumberNode::new(1111),
        NumberNode::new(2222),
        '>',
        PrintNode::new("impossible"));

    assert_eq!(as_node::<NumberNode>(&node.left), &NumberNode::new(1111));
    assert_eq!(as_node::<NumberNode>(&node.right), &NumberNode::new(2222));
    assert_eq!(node.relop, '>');
    assert_eq!(as_node::<PrintNode>(&node.then), &PrintNode::new("impossible"));

    assert_eq!(test_eval(&node), "eval_if")
}

fn as_node<T>(node: &Box<dyn Node>) -> &T where T: Node {
    node.as_any().downcast_ref::<T>().unwrap()
}
