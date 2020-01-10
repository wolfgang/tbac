use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;

#[derive(Default)]
pub struct CodeGenerator {}

impl CodeGenerator {

}

impl NodeEvaluator for CodeGenerator {
    fn eval_print(&self, node: &PrintNode) -> String {
        format!("console.log(\"{}\")", node.string_param)
    }

    fn eval_number(&self, node: &NumberNode) -> String {
        node.value.to_string()
    }

    fn eval_if(&self, node: &IfNode) -> String {
        format!("if ({} {} {}) {{ {} }}",
                node.left.eval(self),
                node.relop,
                node.right.eval(self),
                node.then.eval(self))
    }
}
