use crate::_tests::helpers::assert_print_node;
use crate::tokenizer::Token;
use crate::_tests::parser::helpers::parse;

#[test]
fn parse_print_statement() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];
    let node = parse(&tokens).unwrap();

    assert_eq!(node.children.len(), 1);
    assert_print_node(&node.children[0], "hello");
}
