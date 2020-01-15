use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::node::Node;
use crate::parser::let_node::LetNode;
use crate::parser::var_node::VarNode;
use crate::parser::string_node::StringNode;
use crate::parser::expression_node::ExpressionNode;

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn generate(&self, root: &SequenceNode) -> String {
        root.eval(self)
    }
}

impl NodeEvaluator for CodeGenerator {
    fn eval_print(&self, node: &PrintNode) -> String {
        let mut result = String::with_capacity(256);
        for param in &node.params {
            result.push_str(&format!("console.log({});", param.eval(self)));
        }
        result.push('\n');
        result
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

    fn eval_let(&self, node: &LetNode) -> String {
        format!("{} = {};\n", node.var, node.value.eval(self))
    }

    fn eval_var(&self, node: &VarNode) -> String {
        node.var_name.to_string()
    }

    fn eval_string(&self, node: &StringNode) -> String {
        format!("'{}'", node.value.clone())
    }

    fn eval_expression(&self, _node: &ExpressionNode) -> String {
        unimplemented!()
    }
}
