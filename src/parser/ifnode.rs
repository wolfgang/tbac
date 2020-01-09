use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

pub struct IfNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub relop: char,
    pub then: Box<dyn Node>,
}

impl IfNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>, relop: char, then: Box<dyn Node>) -> Box<Self> {
        Box::new(Self { left, right, relop, then })
    }
}

impl Node for IfNode {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) -> String {
        evaluator.eval_if(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}