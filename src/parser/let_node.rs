use std::any::Any;

use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;

pub struct LetNode {
    pub var: char,
    pub value: NodeBox,
    pub line: u32,
}

impl LetNode {
    pub fn new(var: char, value: NodeBox, line: u32) -> Box<Self> {
        Box::new(Self { var, value, line })
    }
}

impl Node for LetNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String { evaluator.eval_let(self) }
    fn as_any(&self) -> &dyn Any { self }
    fn line(&self) -> u32 { self.line }
}

