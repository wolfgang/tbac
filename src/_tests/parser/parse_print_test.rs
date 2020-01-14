use crate::tokenizer::Token;
use crate::_tests::parser::helpers::*;

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
