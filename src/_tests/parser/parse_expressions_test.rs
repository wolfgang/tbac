use crate::tokenizer::Token;
use crate::_tests::parser::helpers::{parse, as_node, assert_number_node, assert_var_node};
use crate::parser::expression_node::ExpressionNode;
use crate::parser::let_node::LetNode;
use crate::parser::sequence_node::SequenceNode;

#[test]
fn parses_binary_expressions_with_numbers_in_let() {
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

#[test]
fn parses_binary_expressions_with_left_var_in_let() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::var('B'),
        Token::termop('+'),
        Token::number(10)
    ];

    let root = parse_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '+');
    assert_var_node(&expression_node.left, 'B');
    assert_number_node(&expression_node.right, 10);
}

#[test]
fn parses_binary_expressions_with_right_var_in_let() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::number(10),
        Token::termop('-'),
        Token::var('B')
    ];

    let root = parse_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '-');
    assert_number_node(&expression_node.left, 10);
    assert_var_node(&expression_node.right, 'B');
}

fn parse_single_node(tokens: &Vec<Token>) -> SequenceNode {
    let root = parse(&tokens).unwrap();
    assert_eq!(root.children.len(), 1);
    root
}

fn get_let_expression(root: &SequenceNode) -> &ExpressionNode {
    let let_node = verify_let_node(&root);
    as_node::<ExpressionNode>(&let_node.value)

}

fn verify_let_node(root: &SequenceNode) -> &LetNode {
    let let_node = as_node::<LetNode>(&root.children[0]);
    assert_eq!(let_node.var, 'A');
    let_node
}
