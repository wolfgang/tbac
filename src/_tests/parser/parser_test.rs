use crate::tokenizer::{tokenize, Token};
use crate::parser::ifnode::IfNode;
use crate::_tests::parser::helpers::*;
use crate::_tests::parser::helpers::assert_number_node;

#[test]
fn parse_if_statement() {
    let tokens = tokenize(r#"IF 1111 > 2222 THEN PRINT "hello""#).unwrap();
    let root = parse_as_single_node(&tokens);

    let if_node = as_node::<IfNode>(&root.children[0]);
    assert_number_node(&if_node.left, 1111);
    assert_number_node(&if_node.right, 2222);
    assert_eq!(if_node.relop, '>');
    assert_print_node(&if_node.then, "hello");
}

#[test]
fn parse_two_statements() {
    let tokens = tokenize(r#"PRINT "hello" PRINT "world""#).unwrap();
    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 2);

    assert_print_node(&node.children[0], "hello");
    assert_print_node(&node.children[1], "world");
}

#[test]
fn parse_if_statement_with_if_statement_in_then() {
    let tokens = tokenize(r#"IF 1111 > 2222 THEN IF 3333 < 4444 THEN PRINT "hello""#).unwrap();
    let root = parse_as_single_node(&tokens);

    let if_node = as_node::<IfNode>(&root.children[0]);
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
    let tokens = tokenize(r#"IF A > B THEN PRINT "hello""#).unwrap();
    let root = parse_as_single_node(&tokens);

    let if_node = as_node::<IfNode>(&root.children[0]);
    assert_var_node(&if_node.left, 'A');
    assert_var_node(&if_node.right, 'B');
    assert_eq!(if_node.relop, '>');
    assert_print_node(&if_node.then, "hello");
}


#[test]
fn parsing_statements_with_line_numbers_produces_nodes_with_line_numbers() {
    let tokens = tokenize(r#"
        10 PRINT "hello"
        PRINT "world"
        20 LET A = 1
        30 IF A > 1 THEN PRINT "yo"
    "#).unwrap();
    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 4);

    assert_eq!(node.children[0].line(), 10);
    assert_eq!(node.children[1].line(), 0);
    assert_eq!(node.children[2].line(), 20);
    assert_eq!(node.children[3].line(), 30);
}


#[test]
fn return_error_if_if_token_not_followed_by_expression() {
    let tokens = tokenize("IF PRINT").unwrap();

    assert_parse_error(parse(&tokens), "Expected Var but got Statement");
}

#[test]
fn return_error_if_first_token_is_not_command() {
    let tokens = tokenize(r#""hello""#).unwrap();
    assert_parse_error(parse(&tokens), "Expected command token but got StringLiteral");

}


#[test]
fn return_error_if_then_branch_is_not_a_command() {
    let tokens = tokenize(r#"IF A > B THEN "hello""#).unwrap();
    assert_parse_error(parse(&tokens), "Expected command token but got StringLiteral");
}

#[test]
fn return_error_if_let_not_followed_by_equal_sign() {
    let tokens = tokenize(r#"LET A < 1234"#).unwrap();
    assert_parse_error(parse(&tokens), "Expected = but got <");
}

#[test]
fn return_error_if_if_has_not_enough_parts() {
    let tokens = tokenize(r#"IF A >"#).unwrap();
    assert_parse_error(parse(&tokens), "Expected Var but reached the end");
}


#[test]
fn return_error_if_statement_is_invalid() {
    let tokens = vec![Token::statement("NOTPRINT")];
    assert_parse_error(parse(&tokens), "Invalid statement 'NOTPRINT'");
}
