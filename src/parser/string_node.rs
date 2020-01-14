use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

#[derive(PartialEq, Debug)]
pub struct StringNode {
    pub value: String
}

impl StringNode {
    pub fn new(value: &str) -> Box<Self> {
        Box::new(Self { value: value.to_string() })
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