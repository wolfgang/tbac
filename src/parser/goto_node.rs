use std::any::Any;

use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;

pub struct GotoNode {
    pub destination: NodeBox,
    line: u32,
}

impl GotoNode {
    pub fn new(destination: NodeBox, line: u32) -> Box<Self> {
        Box::new(Self { destination, line })
    }
}

impl Node for GotoNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_goto(self)
    }
    fn as_any(&self) -> &dyn Any { self }
    fn line(&self) -> u32 { self.line }
}