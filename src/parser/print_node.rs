use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub struct PrintNode {
    pub string_param: String,
    pub params: Vec<Box<dyn Node>>
}

impl PrintNode {
    pub fn new(param: &str) -> Box<Self> {
        Box::new(Self {
            string_param: param.to_string(),
            params: Vec::with_capacity(8)
        })
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

