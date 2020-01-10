use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::ifnode::IfNode;

struct CodeGenerator {

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

    fn eval_number(&mut self, node: &NumberNode) -> String {
        "".to_string()
    }

    fn eval_if(&mut self, node: &IfNode) -> String {
        "".to_string()
    }
}

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello world"));

    let mut code_generator = CodeGenerator::new();

    let code = root.eval(&mut code_generator);

    assert_eq!(code, r#"console.log("hello world")"#)
}