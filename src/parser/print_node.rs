use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::string_node::StringNode;

pub struct PrintNode {
    pub params: Vec<Box<dyn Node>>
}

impl PrintNode {
    pub fn with_string_param(param: &str) -> Box<Self> {
        let mut params: Vec<Box<dyn Node>> = Vec::with_capacity(8);
        params.push(StringNode::new(param));
        Box::new(Self { params })
    }

    pub fn new() -> Box<Self> {
        Box::new(Self { params: Vec::with_capacity(8)})
    }

    pub fn add_param(&mut self, param: Box<dyn Node>) {
        self.params.push(param)
    }
}

impl Node for PrintNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_print(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

