use crate::_tests::parser::helpers::*;
use crate::parser::expression_node::ExpressionNode;
use crate::parser::let_node::LetNode;
use crate::parser::sequence_node::SequenceNode;
use crate::tokenizer::tokenize;

#[test]
fn parses_binary_expressions_with_numbers_in_let() {
    let tokens = tokenize("LET A = 10 + 20").unwrap();
    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '+');
    assert_number_node(&expression_node.left, 10);
    assert_number_node(&expression_node.right, 20);

}

#[test]
fn parses_binary_expressions_with_left_var_in_let() {
    let tokens = tokenize("LET A = B + 10").unwrap();
    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '+');
    assert_var_node(&expression_node.left, 'B');
    assert_number_node(&expression_node.right, 10);
}

#[test]
fn parses_binary_expressions_with_right_var_in_let() {
    let tokens = tokenize("LET A = 10 - B").unwrap();
    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '-');
    assert_number_node(&expression_node.left, 10);
    assert_var_node(&expression_node.right, 'B');
}

#[test]
fn parses_expression_with_factor() {
    let tokens = tokenize("LET A = 10*20+30").unwrap();
    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '+');
    assert_number_node(&expression_node.right, 30);

    let left_expression = as_node::<ExpressionNode>(&expression_node.left);
    assert_eq!(left_expression.op, '*');
    assert_number_node(&left_expression.left, 10);
    assert_number_node(&left_expression.right, 20);
}

#[test]
fn parses_expression_with_multiple_factors() {
    let tokens = tokenize("LET A = 10*20*30").unwrap();

    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '*');
    assert_number_node(&expression_node.left, 10);

    let left_expression = as_node::<ExpressionNode>(&expression_node.right);
    assert_eq!(left_expression.op, '*');
    assert_number_node(&left_expression.left, 20);
    assert_number_node(&left_expression.right, 30);
}

#[test]
fn parses_expression_with_brackets() {
    let tokens = tokenize("LET A = 10 * (20 + 30)").unwrap();

    let root = parse_as_single_node(&tokens);
    let expression_node = get_let_expression(&root);

    assert_eq!(expression_node.op, '*');
    assert_number_node(&expression_node.left, 10);

    let left_expression = as_node::<ExpressionNode>(&expression_node.right);
    assert_eq!(left_expression.op, '+');
    assert_number_node(&left_expression.left, 20);
    assert_number_node(&left_expression.right, 30);
}



#[test]
fn return_error_if_expression_is_incomplete() {
    let tokens = tokenize("LET A = 10 -").unwrap();
    assert_parse_error(parse(&tokens), "Expected Var but reached the end");
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
