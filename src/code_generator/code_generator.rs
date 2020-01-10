use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::node::Node;
use crate::parser::let_node::LetNode;

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn generate(&self, root: &SequenceNode) -> String {
        root.eval(self)
    }
}

impl NodeEvaluator for CodeGenerator {
    fn eval_print(&self, node: &PrintNode) -> String {
        format!("console.log('{}');\n", node.string_param)
    }

    fn eval_number(&self, node: &NumberNode) -> String {
        node.value.to_string()
    }

    fn eval_if(&self, node: &IfNode) -> String {
        format!("if ({} {} {}) {{\n  {}}}\n",
                node.left.eval(self),
                node.relop,
                node.right.eval(self),
                node.then.eval(self))
    }

    fn eval_let(&self, _node: &LetNode) -> String {
        "let not implemented yet".to_string()
    }
}
