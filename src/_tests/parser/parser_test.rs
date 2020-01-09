use crate::tokenizer::Token;
use crate::parser::parser::Parser;
use crate::_tests::helpers::*;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::ifnode::IfNode;

#[test]
fn parse_print_node() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];
    let node = parse(&tokens).unwrap();

    assert_eq!(node.children.len(), 1);
    assert_print_node(&node.children[0], "hello");
}


#[test]
fn parse_if_node() {
    let tokens = vec![
        Token::iff(),
        Token::number("1111"),
        Token::relop('>'),
        Token::number("2222"),
        Token::then(),
        Token::print(),
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.children.len(), 1);

    let if_node = as_node::<IfNode>(&node.children[0]);
    assert_number_node(&if_node.left, 1111);
    assert_number_node(&if_node.right, 2222);
    assert_print_node(&if_node.then, "hello");
}

#[test]
fn parse_two_statements() {
    let tokens = vec![
        Token::print(),
        Token::string("hello"),
        Token::print(),
        Token::string("world")
    ];

    let result = parse(&tokens);
    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.children.len(), 2);

    assert_print_node(&node.children[0], "hello");
    assert_print_node(&node.children[1], "world");
}

#[test]
fn return_error_if_if_token_not_followed_by_number() {
    let tokens = vec![
        Token::iff(),
        Token::string("abcd"),
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected NUMBER but got STRING");
}

#[test]
fn return_error_if_first_token_is_not_command() {
    let tokens = vec![
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected command token here");

}

#[test]
fn return_error_if_print_has_non_string_argument() {
    let tokens = vec![
        Token::print(),
        Token::then()
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected STRING but got THEN");

}

fn parse(tokens: &Vec<Token>) -> Result<SequenceNode, String> {
    Parser::new(tokens).parse()
}

fn assert_parse_error(result: Result<SequenceNode, String>, error: &str) {
    assert!(result.is_err());
    assert_eq!(result.err(), Some(error.to_string()))
}