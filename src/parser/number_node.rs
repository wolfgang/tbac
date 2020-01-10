use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

#[derive(PartialEq, Debug)]
pub struct NumberNode {
    pub value: i32
}

impl NumberNode {
    pub fn new(value: i32) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl Node for NumberNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_number(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}