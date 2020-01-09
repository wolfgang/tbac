use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;

pub fn assert_number_node(node: &Box<dyn Node>, number: i32) {
    assert_is_node::<NumberNode>(node);
    assert_eq!(as_node::<NumberNode>(node), &*NumberNode::new(number));
}

pub fn assert_print_node(node: &Box<dyn Node>, param: &str) {
    assert_is_node::<PrintNode>(node);
    assert_eq!(as_node::<PrintNode>(node), &*PrintNode::new(param));
}


pub fn as_node<T>(node: &Box<dyn Node>) -> &T where T: Node {
    node.as_any().downcast_ref::<T>().unwrap()
}

pub fn assert_is_node<T>(node: &Box<dyn Node>)  where T: Node {
    assert!(node.as_any().downcast_ref::<T>().is_some())
}


