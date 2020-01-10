use crate::parser::var_node::VarNode;
use crate::_tests::parser::test_node_evaluator::test_eval;

#[test]
fn construct_and_eval() {
    let node = VarNode::new('Y');
    assert_eq!(node.var_name, 'Y');
    assert_eq!(test_eval(&*node), "eval_var")
}