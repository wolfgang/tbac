use crate::parser::node::Node;

pub fn as_node<T>(node: &Box<dyn Node>) -> &T where T: Node {
    node.as_any().downcast_ref::<T>().unwrap()
}
