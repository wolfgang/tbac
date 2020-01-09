use std::any::Any;

use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;

pub struct IfNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub relop: char,
    pub then: Box<dyn Node>,
}

impl IfNode {
    pub fn new<T, U, V>(left: T, right: U, relop: char, then: V) -> Self
        where T: Node + 'static, U: Node + 'static, V: Node + 'static {
        Self { left: Box::new(left), right: Box::new(right), relop, then: Box::new(then) }
    }

    pub fn new2<T, U, V>(left: T, right: U, relop: char, then: Box<V>) -> Self
        where T: Node + 'static, U: Node + 'static, V: Node + 'static {
        Self { left: Box::new(left), right: Box::new(right), relop, then }
    }

    pub fn new3(left: Box<dyn Node>, right: Box<dyn Node>, relop: char, then: Box<dyn Node>) -> Self {
        Self { left, right, relop, then }
    }
}

impl Node for IfNode {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) -> String {
        evaluator.eval_if(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}