use crate::tokenizer::Token;
use crate::_tests::parser::helpers::*;
use crate::parser::let_node::LetNode;

#[test]
fn parse_let_statement() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::number(1234)
    ];
    let node = parse_as_single_node(&tokens);

    let let_node = as_node::<LetNode>(&node.children[0]);
    assert_eq!(let_node.var, 'A');
    assert_number_node(&let_node.value, 1234);
}

#[test]
fn parse_let_with_var_on_the_right() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::var('B')
    ];
    let root = parse_as_single_node(&tokens);

    let let_node = as_node::<LetNode>(&root.children[0]);
    assert_eq!(let_node.var, 'A');
    assert_var_node(&let_node.value, 'B');

}
