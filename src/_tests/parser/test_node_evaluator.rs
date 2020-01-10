use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::node::Node;

pub fn test_eval(node: &dyn Node) -> String {
    let mut evaluator = TestNodeEvaluator::new();
    node.eval(&mut evaluator)
}

pub struct TestNodeEvaluator {
    pub(crate) code: Vec<String>
}

impl TestNodeEvaluator {
    pub fn new() -> Self {
        Self { code: Vec::with_capacity(32) }
    }

    pub fn assert_code(&self, expected_code: Vec<&str>) {
        assert_eq!(self.code, expected_code);
    }
}

impl NodeEvaluator for TestNodeEvaluator {
    fn eval_print(&self, _: &PrintNode) -> String {
        "eval_print".to_string()
    }

    fn eval_number(&self, _: &NumberNode) -> String {
        "eval_number".to_string()
    }

    fn eval_if(&self, _: &IfNode) -> String {
        "eval_if".to_string()
    }
}


