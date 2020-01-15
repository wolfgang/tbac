use crate::parser::node::{NodeBox, Node};
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub struct ExpressionNode {
    pub op: char,
    pub left: NodeBox,
    pub right: NodeBox,
}

impl ExpressionNode {
    pub fn new(op: char, left: NodeBox, right: NodeBox) -> Self {
        Self { op, left, right }
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