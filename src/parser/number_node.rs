use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

pub struct NumberNode {
    pub value: i32
}

impl NumberNode {
    pub fn new(value: i32) -> Self {
        Self {value}
    }
}

impl Node for NumberNode {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) -> String {
        evaluator.eval_number(&self)
    }
}