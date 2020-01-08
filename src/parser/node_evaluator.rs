use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;

pub trait NodeEvaluator {
    fn eval_print(&mut self, node: &PrintNode) -> String;
    fn eval_number(&mut self, node: &NumberNode) -> String;
}
