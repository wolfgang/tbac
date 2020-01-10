use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

#[derive(PartialEq, Debug)]
pub struct PrintNode {
    pub string_param: String
}

impl PrintNode {
    pub fn new(param: &str) -> Box<Self> {
        Box::new(Self { string_param: param.to_string() })
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

