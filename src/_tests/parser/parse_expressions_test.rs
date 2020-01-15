use crate::tokenizer::Token;
use crate::_tests::parser::helpers::{parse, as_node, assert_number_node};
use crate::parser::expression_node::ExpressionNode;
use crate::parser::let_node::LetNode;

#[test]
fn parses_binary_expressions_in_let() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::number(10),
        Token::termop('+'),
        Token::number(20)
    ];

    let root = parse(&tokens).unwrap();

    assert_eq!(root.children.len(), 1);

    let let_node = as_node::<LetNode>(&root.children[0]);
    assert_eq!(let_node.var, 'A');
    let expression_node = as_node::<ExpressionNode>(&let_node.value);
    assert_eq!(expression_node.op, '+');
    assert_number_node(&expression_node.left, 10);
    assert_number_node(&expression_node.right, 20);

}