use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub struct LetNode {
    pub var: char,
    pub value: NodeBox
}

impl LetNode {
    pub fn new(var: char, value: NodeBox) -> Box<Self> {
        Box::new(Self {var, value})
    }
}

impl Node for LetNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_let(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

