use crate::parser::node_evaluator::NodeEvaluator;
use std::any::Any;

pub trait Node : Any {
    fn eval(&self, evaluator: &dyn NodeEvaluator) -> String;
    fn as_any(&self) -> &dyn Any;
}
