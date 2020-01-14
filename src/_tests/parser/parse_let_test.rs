use crate::tokenizer::Token;
use crate::_tests::parser::helpers::{as_node, assert_number_node, parse, assert_var_node};
use crate::parser::let_node::LetNode;

#[test]
fn parse_let_statement() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::bin_op('='),
        Token::number(1234)
    ];
    let result = parse(&tokens);
    assert!(result.is_ok());

    let node = result.unwrap();
    assert_eq!(node.children.len(), 1);

    let let_node = as_node::<LetNode>(&node.children[0]);
    assert_eq!(let_node.var, 'A');
    assert_number_node(&let_node.value, 1234);
}

#[test]
fn parse_let_with_var_on_the_right() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::bin_op('='),
        Token::var('B')
    ];
    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 1);

    let let_node = as_node::<LetNode>(&node.children[0]);
    assert_eq!(let_node.var, 'A');
    assert_var_node(&let_node.value, 'B');

}
