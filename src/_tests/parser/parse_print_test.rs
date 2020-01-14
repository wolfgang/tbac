use crate::tokenizer::Token;
use crate::_tests::parser::helpers::*;
use crate::parser::print_node::PrintNode;

#[test]
fn parses_with_single_string_parameter() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];
    let node = parse(&tokens).unwrap();

    assert_eq!(node.children.len(), 1);
    assert_print_node(&node.children[0], "hello");
}


#[test]
fn parses_with_single_number_expression() {
    let tokens = vec![
        Token::print(),
        Token::number(1234)
    ];
    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 1);
    assert_print_node(&node.children[0], "1234");

}

#[ignore]
#[test]
fn parses_with_multiple_strings() {
    let tokens = vec![
        Token::print(),
        Token::string("hello"),
        Token::comma(),
        Token::string("world"),
        Token::comma(),
        Token::string("the end")
    ];
    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 1);
    assert_is_node::<PrintNode>(&node.children[0]);

}

#[test]
fn return_error_if_print_has_non_string_argument() {
    let tokens = vec![
        Token::print(),
        Token::then()
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected STRING or NUMBER but got THEN");
}

#[test]
fn return_error_if_print_has_no_argument() {
    let tokens = vec![
        Token::print()
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Premature end of token stream");

}
