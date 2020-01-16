use crate::tokenizer::Token;
use crate::_tests::parser::helpers::{parse_as_single_node, as_node, assert_number_node};
use crate::parser::goto_node::GotoNode;
use crate::parser::node::Node;

#[test]
fn parses_goto_statement() {
    let tokens = vec![
        Token::number(10),
        Token::goto(),
        Token::number(20)
    ];
    let root = parse_as_single_node(&tokens);
    let goto_node = as_node::<GotoNode>(&root.children[0]);
    assert_eq!(goto_node.line(), 10);
    assert_number_node(&goto_node.destination, 20);
}
