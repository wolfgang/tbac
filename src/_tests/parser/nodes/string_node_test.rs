use crate::_tests::parser::helpers::test_eval;
use crate::parser::string_node::StringNode;

#[test]
fn construct_and_eval() {
    let node = StringNode::new("abcd".to_string());
    assert_eq!(node.value, "abcd");
    assert_eq!(test_eval(&*node), "eval_string");
}
