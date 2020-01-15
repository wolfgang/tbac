use crate::tokenizer::tokenize;
use crate::_tests::parser::helpers::*;
use crate::parser::let_node::LetNode;

#[test]
fn parse_let_statement() {
    let tokens = tokenize("LET A = 1234").unwrap();
    let node = parse_as_single_node(&tokens);
    let let_node = as_node::<LetNode>(&node.children[0]);

    assert_eq!(let_node.var, 'A');
    assert_number_node(&let_node.value, 1234);
}

#[test]
fn parse_let_with_var_on_the_right() {
    let tokens = tokenize("LET A = B").unwrap();
    let root = parse_as_single_node(&tokens);

    let let_node = as_node::<LetNode>(&root.children[0]);
    assert_eq!(let_node.var, 'A');
    assert_var_node(&let_node.value, 'B');
}
