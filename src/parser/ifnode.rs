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

        Self { left: Box::new(left), right: Box::new(right), relop, then: Box::new(then)}
    }
}

impl Node for IfNode {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) -> String {
        evaluator.eval_if(&self)
    }
}