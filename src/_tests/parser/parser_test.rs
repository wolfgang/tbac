use crate::tokenizer::Token;
use crate::parser::ifnode::IfNode;
use crate::_tests::parser::helpers::*;

#[test]
fn parse_if_statement() {
    let tokens = vec![
        Token::iff(),
        Token::number(1111),
        Token::bin_op('>'),
        Token::number(2222),
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
    assert_eq!(if_node.relop, '>');
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
fn parse_if_statement_with_if_statement_in_then() {
    let tokens = vec![
        Token::iff(),
        Token::number(1111),
        Token::bin_op('>'),
        Token::number(2222),
        Token::then(),
        Token::iff(),
        Token::number(3333),
        Token::bin_op('<'),
        Token::number(4444),
        Token::then(),
        Token::print(),
        Token::string("hello"),
    ];

    let result = parse(&tokens);
    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.children.len(), 1);

    let if_node = as_node::<IfNode>(&node.children[0]);
    assert_number_node(&if_node.left, 1111);
    assert_number_node(&if_node.right, 2222);
    assert_eq!(if_node.relop, '>');


    let then_node = as_node::<IfNode>(&if_node.then);
    assert_number_node(&then_node.left, 3333);
    assert_number_node(&then_node.right, 4444);
    assert_eq!(then_node.relop, '<');
    assert_print_node(&then_node.then, "hello");
}

#[test]
fn parse_if_statement_with_vars() {
    let tokens = vec![
        Token::iff(),
        Token::var('A'),
        Token::bin_op('>'),
        Token::var('B'),
        Token::then(),
        Token::print(),
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert_eq!(result.as_ref().err(), None);

    let node = result.unwrap();
    assert_eq!(node.children.len(), 1);

    let if_node = as_node::<IfNode>(&node.children[0]);
    assert_var_node(&if_node.left, 'A');
    assert_var_node(&if_node.right, 'B');
    assert_eq!(if_node.relop, '>');
    assert_print_node(&if_node.then, "hello");
}



#[test]
fn return_error_if_if_token_not_followed_by_expression() {
    let tokens = vec![
        Token::iff(),
        Token::print(),
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected Var but got Print");
}

#[test]
fn return_error_if_first_token_is_not_command() {
    let tokens = vec![
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected command token but got StringLiteral");

}


#[test]
fn return_error_if_then_branch_is_not_a_command() {
    let tokens = vec![
        Token::iff(),
        Token::number(1111),
        Token::bin_op('>'),
        Token::number(2222),
        Token::then(),
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected command token but got StringLiteral");
}

#[test]
fn return_error_if_let_not_followed_by_equal_sign() {
    let tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::bin_op('<'),
        Token::number(1234)
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected = but got <");
}

#[test]
fn return_error_if_if_has_not_enough_parts() {
    let tokens = vec![
        Token::iff()
    ];

    let result = parse(&tokens);
    assert_parse_error(result, "Expected Var but reached the end");

}

