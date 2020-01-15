use crate::tokenizer::Token;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::parser::Parser;
use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::print_node::PrintNode;
use crate::parser::number_node::NumberNode;
use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::var_node::VarNode;
use crate::parser::string_node::StringNode;
use crate::parser::expression_node::ExpressionNode;

pub fn parse_as_single_node(tokens: &Vec<Token>) -> SequenceNode {
    let root = parse(&tokens).unwrap();
    assert_eq!(root.children.len(), 1);
    root
}

pub fn parse(tokens: &Vec<Token>) -> Result<SequenceNode, String> {
    Parser::new(tokens).parse()
}

pub fn assert_parse_error(result: Result<SequenceNode, String>, error: &str) {
    assert!(result.is_err());
    assert_eq!(result.err(), Some(error.to_string()))
}

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

    fn eval_string(&self, _node: &StringNode) -> String {
        "eval_string".to_string()
    }

    fn eval_expression(&self, _node: &ExpressionNode) -> String {
        "eval_expression".to_string()
    }
}

pub fn assert_number_node(node: &NodeBox, number: i32) {
    assert_eq!(as_node::<NumberNode>(node), &*NumberNode::new(number));
}

pub fn assert_string_node(node: &NodeBox, value: &str) {
    assert_eq!(as_node::<StringNode>(node), &*StringNode::new(value));
}

pub fn assert_print_node(node: &NodeBox, param: &str) {
    assert_is_node::<PrintNode>(node);
    let print_node = as_node::<PrintNode>(node);
    assert_eq!(as_node::<StringNode>(&print_node.params[0]).value, param);
}

pub fn assert_var_node(node: &NodeBox, var_name: char) {
    assert_eq!(as_node::<VarNode>(node), &*VarNode::new(var_name));
}

pub fn as_node<T>(node: &NodeBox) -> &T where T: Node {
    assert_is_node::<T>(node);
    node.as_any().downcast_ref::<T>().unwrap()
}

pub(crate) fn assert_is_node<T>(node: &NodeBox)  -> &T where T: Node {
    let down_cast = node.as_any().downcast_ref::<T>();
    assert!(down_cast.is_some());
    down_cast.unwrap()
}
