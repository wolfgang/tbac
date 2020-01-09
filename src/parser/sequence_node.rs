use crate::parser::node::Node;

pub struct SequenceNode {
    pub children: Vec<Box<dyn Node>>
}

impl SequenceNode {
    pub fn new() -> Self {
        Self { children: Vec::with_capacity(16) }
    }

    pub fn add<T>(&mut self, node: T) where T: Node + 'static {
        self.children.push(Box::new(node))
    }
}