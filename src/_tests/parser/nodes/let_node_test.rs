use crate::parser::number_node::NumberNode;
use crate::parser::let_node::LetNode;
use crate::_tests::parser::helpers::*;
use crate::parser::node::Node;

#[test]
fn construct_and_eval() {
    let node = LetNode::new('X', NumberNode::new(1234), 1111);

    assert_eq!(node.var, 'X');
    assert_eq!(node.line(), 1111);
    assert_number_node(&node.value, 1234);

    assert_eq!(test_eval(&*node), "eval_let")

}