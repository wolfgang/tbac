use std::any::Any;

use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;

pub struct IfNode {
    pub left: NodeBox,
    pub right: NodeBox,
    pub relop: char,
    pub then: NodeBox,
    pub line: u32,
}

impl IfNode {
    pub fn new(left: NodeBox,
               right: NodeBox,
               relop: char,
               then: NodeBox,
               line: u32) -> Box<Self> {
        Box::new(Self { left, right, relop, then, line })
    }
}

impl Node for IfNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String { evaluator.eval_if(&self) }
    fn as_any(&self) -> &dyn Any { self }
    fn line(&self) -> u32 { self.line }
}