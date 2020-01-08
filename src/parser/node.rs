use crate::parser::node_evaluator::NodeEvaluator;

pub trait Node {
    fn eval(&self, evaluator: &mut dyn NodeEvaluator) -> String;
}
