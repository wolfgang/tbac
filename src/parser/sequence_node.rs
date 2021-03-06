use std::any::Any;

use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;

pub struct SequenceNode {
    pub children: Vec<NodeBox>
}

impl SequenceNode {
    pub fn new() -> Self {
        Self { children: Vec::with_capacity(16) }
    }

    pub fn add(&mut self, node: NodeBox)  {
        self.children.push(node)
    }

}

impl Node for SequenceNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_sequence(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}