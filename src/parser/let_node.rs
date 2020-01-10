use crate::parser::node::Node;

pub struct LetNode {
    pub var: char,
    pub value: Box<dyn Node>
}

impl LetNode {
    pub fn new(var: char, value: Box<dyn Node>) -> Self {
        Self {var, value}
    }
}

