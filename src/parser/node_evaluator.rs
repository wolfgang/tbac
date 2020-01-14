use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::var_node::VarNode;
use crate::parser::string_node::StringNode;

pub trait NodeEvaluator {
    fn eval_print(&self, node: &PrintNode) -> String;
    fn eval_number(&self, node: &NumberNode) -> String;
    fn eval_if(&self, node: &IfNode) -> String;
    fn eval_let(&self, node: &LetNode) -> String;
    fn eval_var(&self, node: &VarNode) -> String;
    fn eval_string(&self, node: &StringNode) -> String;
}
