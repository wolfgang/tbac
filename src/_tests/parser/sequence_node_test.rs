use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::tokenizer::token::TokenType::PRINT;
//use crate::parser::print_node::PrintNode;

#[test]
fn construct_and_eval() {
    let mut node = SequenceNode::new();
    node.add(NumberNode::new(1111));
    node.add(PrintNode::new("hello"));

    assert_eq!(node.children.len(), 2);

    assert_eq!(as_node::<NumberNode>(&node.children[0]), &NumberNode::new(1111));
    assert_eq!(as_node::<PrintNode>(&node.children[1]), &PrintNode::new("hello"));
}

fn as_node<T>(node: &Box<dyn Node>) -> &T where T: Node {
    node.as_any().downcast_ref::<T>().unwrap()
}

