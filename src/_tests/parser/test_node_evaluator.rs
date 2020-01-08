use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;

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
    fn eval_print(&mut self, node: &PrintNode) -> String {
        format!("print {}", node.string_param)
    }

    fn eval_number(&mut self, node: &NumberNode) -> String {
        format!("{}", node.value)
    }

    fn eval_if(&mut self, node: &IfNode) -> String {
        format!("if {} {} {} {}",
                node.left.eval(self),
                node.relop,
                node.right.eval(self),
                node.then.eval(self))
    }
}


