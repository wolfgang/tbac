use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

pub struct PrintNode {
    pub string_param: String
}

impl PrintNode {
    pub fn new(param: &str) -> Self {
        Self { string_param: param.to_string() }
    }
}

impl Node for PrintNode {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) {
        evaluator.eval_print(&self);
    }
}

