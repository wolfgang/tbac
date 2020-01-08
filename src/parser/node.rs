use crate::parser::node_evaluator::NodeEvaluator;

pub trait Node {
    fn eval(&self, code_generator: &mut dyn NodeEvaluator);
}
