use crate::parser::ifnode::IfNode;
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::node::Node;
use crate::parser::let_node::LetNode;
use crate::parser::var_node::VarNode;

pub fn test_eval(node: &dyn Node) -> String {
    let mut evaluator = FakeNodeEvaluator::new();
    node.eval(&mut evaluator)
}

pub struct FakeNodeEvaluator {
    pub(crate) code: Vec<String>
}

impl FakeNodeEvaluator {
    pub fn new() -> Self {
        Self { code: Vec::with_capacity(32) }
    }

    pub fn assert_code(&self, expected_code: Vec<&str>) {
        assert_eq!(self.code, expected_code);
    }
}

impl NodeEvaluator for FakeNodeEvaluator {
    fn eval_print(&self, _: &PrintNode) -> String {
        "eval_print".to_string()
    }

    fn eval_number(&self, _: &NumberNode) -> String {
        "eval_number".to_string()
    }

    fn eval_if(&self, _: &IfNode) -> String {
        "eval_if".to_string()
    }

    fn eval_let(&self, _: &LetNode) -> String {
        "eval_let".to_string()
    }

    fn eval_var(&self, _node: &VarNode) -> String {
        "eval_var".to_string()
    }
}