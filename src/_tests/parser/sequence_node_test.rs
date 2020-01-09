use crate::parser::number_node::NumberNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::_tests::parser::test_node_evaluator::test_eval;
use crate::_tests::helpers::as_node;

#[test]
fn construct_and_eval() {
    let mut node = SequenceNode::new();
    node.add2(NumberNode::new(1111));
    node.add2(Box::new(PrintNode::new("hello")));

    assert_eq!(node.children.len(), 2);

    assert_eq!(as_node::<NumberNode>(&node.children[0]), &*NumberNode::new(1111));
    assert_eq!(as_node::<PrintNode>(&node.children[1]), &PrintNode::new("hello"));

    assert_eq!(test_eval(&node), "eval_numbereval_print");

}


