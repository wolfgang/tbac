use std::any::Any;

use crate::parser::node::{NodeBox, Node};
use crate::parser::node_evaluator::NodeEvaluator;

pub struct IfNode {
    pub left: NodeBox,
    pub right: NodeBox,
    pub relop: char,
    pub then: NodeBox,
}

impl IfNode {
    pub fn new(left: NodeBox, right: NodeBox, relop: char, then: NodeBox) -> Box<Self> {
        Box::new(Self { left, right, relop, then })
    }
}

impl Node for IfNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_if(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}