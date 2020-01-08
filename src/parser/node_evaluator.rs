use crate::parser::print_node::PrintNode;

pub trait NodeEvaluator {
    fn eval_print(&mut self, node: &PrintNode);
}
