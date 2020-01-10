use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;


pub struct SequenceNode {
    pub children: Vec<Box<dyn Node>>
}

impl SequenceNode {
    pub fn new() -> Self {
        Self { children: Vec::with_capacity(16) }
    }

    pub fn add(&mut self, node: Box<dyn Node>)  {
        self.children.push(node)
    }

}

impl Node for SequenceNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        self.children.iter().fold(
            String::with_capacity(512),
            |acc, child| format!("{}{}", acc, child.eval(evaluator)))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}