use crate::tokenizer::Token;
use crate::_tests::parser::helpers::*;
use crate::parser::print_node::PrintNode;

#[test]
fn parses_with_single_string_parameter() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];
    let node = parse_as_single_node(&tokens);
    assert_print_node(&node.children[0], "hello");
}


#[test]
fn parses_with_single_number_expression() {
    let tokens = vec![
        Token::print(),
        Token::number(1234)
    ];
    let node = parse_as_single_node(&tokens);
    let print_node = assert_is_node::<PrintNode>(&node.children[0]);
    assert_eq!(print_node.params.len(), 1);
    assert_number_node(&print_node.params[0], 1234)

}

#[test]
fn parses_with_multiple_params() {
    let tokens = vec![
        Token::print(),
        Token::string("hello"),
        Token::comma(),
        Token::string("world"),
        Token::comma(),
        Token::number(1234)
    ];
    let node = parse_as_single_node(&tokens);
    let print_node = assert_is_node::<PrintNode>(&node.children[0]);
    assert_eq!(print_node.params.len(), 3);
    assert_string_node(&print_node.params[0], "hello");
    assert_string_node(&print_node.params[1], "world");
    assert_number_node(&print_node.params[2], 1234);

}

#[test]
fn parses_print_with_vars() {
    let tokens = vec![
        Token::print(),
        Token::string("hello"),
        Token::comma(),
        Token::var('A'),
    ];
    let node = parse_as_single_node(&tokens);
    let print_node = assert_is_node::<PrintNode>(&node.children[0]);
    assert_eq!(print_node.params.len(), 2);
    assert_string_node(&print_node.params[0], "hello");
    assert_var_node(&print_node.params[1], 'A');
}

#[test]
fn return_error_if_print_has_non_string_argument() {
    let tokens = vec![
        Token::print(),
        Token::then()
    ];

    assert_parse_error(parse(&tokens), "Expected Var but got Then");
}

#[test]
fn return_error_if_print_has_no_argument() {
    let tokens = vec![
        Token::print()
    ];

    assert_parse_error(parse(&tokens), "Expected Var but reached the end");

}
