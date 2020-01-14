use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;
use crate::parser::string_node::StringNode;

pub struct PrintNode {
    pub string_param: String,
    pub params: Vec<Box<dyn Node>>
}

impl PrintNode {
    pub fn new(param: &str) -> Box<Self> {
        let mut params : Vec<Box<dyn Node>> = Vec::with_capacity(8);
        params.push(StringNode::new(param));
        Box::new(Self {
            string_param: param.to_string(),
            params
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

