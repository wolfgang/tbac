use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::var_node::VarNode;

pub fn assert_number_node(node: &Box<dyn Node>, number: i32) {
    assert_eq!(as_node::<NumberNode>(node), &*NumberNode::new(number));
}

pub fn assert_print_node(node: &Box<dyn Node>, param: &str) {
    assert_eq!(as_node::<PrintNode>(node), &*PrintNode::new(param));
}

pub fn assert_var_node(node: &Box<dyn Node>, var_name: char) {
    assert_eq!(as_node::<VarNode>(node), &*VarNode::new(var_name));
}

pub fn as_node<T>(node: &Box<dyn Node>) -> &T where T: Node {
    assert_is_node::<T>(node, );
    node.as_any().downcast_ref::<T>().unwrap()
}

fn assert_is_node<T>(node: &Box<dyn Node>)  where T: Node {
    assert!(node.as_any().downcast_ref::<T>().is_some())
}
