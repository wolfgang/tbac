use crate::parser::node::{NodeBox, Node};
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub struct ExpressionNode {
    pub op: char,
    pub left: NodeBox,
    pub right: NodeBox,
    pub in_brackets: bool
}

impl ExpressionNode {
    pub fn new(op: char, in_brackets: bool, left: NodeBox, right: NodeBox) -> Box<Self> {
        Box::new(Self { op, left, right, in_brackets })
    }
}

impl Node for ExpressionNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_expression(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}