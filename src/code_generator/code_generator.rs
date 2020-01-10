use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::ifnode::IfNode;

pub struct CodeGenerator {

}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeEvaluator for CodeGenerator {
    fn eval_print(&mut self, node: &PrintNode) -> String {
        format!("console.log(\"{}\")", node.string_param)
    }

    fn eval_number(&mut self, _node: &NumberNode) -> String {
        "".to_string()
    }

    fn eval_if(&mut self, _node: &IfNode) -> String {
        "".to_string()
    }
}
