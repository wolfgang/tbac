use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::ifnode::IfNode;

pub trait NodeEvaluator {
    fn eval_print(&mut self, node: &PrintNode) -> String;
    fn eval_number(&mut self, node: &NumberNode) -> String;
    fn eval_if(&mut self, node: &IfNode) -> String;
}
