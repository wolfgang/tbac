use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::print_node::PrintNode;


struct TestNodeEvaluator {
    code: Vec<String>
}

impl TestNodeEvaluator {
    pub fn new() -> Self {
        Self { code: Vec::with_capacity(32) }
    }
}

impl NodeEvaluator for TestNodeEvaluator {
    fn eval_print(&mut self, node: &PrintNode) {
        self.code.push(format!("print {}", node.string_param));
    }
}

#[test]
fn print_node_eval() {
    let node = PrintNode::new("hello");
    let mut code_generator = TestNodeEvaluator::new();

    node.eval(&mut code_generator);

    assert_eq!(code_generator.code, vec!["print hello"]);
}