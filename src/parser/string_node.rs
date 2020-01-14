use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

#[derive(PartialEq, Debug)]
pub struct StringNode {
    pub value: String
}

impl StringNode {
    pub fn new(value: String) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl Node for StringNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_string(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}