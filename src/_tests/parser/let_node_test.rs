use crate::parser::number_node::NumberNode;
use crate::_tests::helpers::assert_number_node;
use crate::parser::let_node::LetNode;

#[test]
fn construct_and_eval() {
    let node = LetNode::new('X', NumberNode::new(1234));

    assert_eq!(node.var, 'X');
    assert_number_node(&node.value, 1234);
}