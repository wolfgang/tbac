use crate::parser::node::Node;
use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub struct VarNode {
    pub var_name: char
}

impl VarNode {
    pub fn new(var_name: char) -> Box<Self> {
        Box::new(Self { var_name })
    }
}

impl Node for VarNode {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String {
        evaluator.eval_var(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}