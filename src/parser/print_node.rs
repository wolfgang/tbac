use std::any::Any;

use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::string_node::StringNode;

pub struct PrintNode {
    pub params: Vec<NodeBox>,
    pub line: u32,
}

impl PrintNode {
    pub fn with_string_param(param: &str) -> Box<Self> {
        let mut params: Vec<NodeBox> = Vec::with_capacity(8);
        params.push(StringNode::new(param));
        Self::with_params(params, 0)
    }

    pub fn new(line: u32) -> Box<Self> {
        Self::with_params(Vec::with_capacity(8), line)
    }

    pub fn add_param(&mut self, param: NodeBox) {
        self.params.push(param)
    }

    fn with_params(params: Vec<NodeBox>, line: u32) -> Box<Self> {
        Box::new(Self { params, line })
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

