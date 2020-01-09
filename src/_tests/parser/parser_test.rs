use crate::tokenizer::Token;
use crate::parser::parser::Parser;
use crate::_tests::helpers::as_node;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::ifnode::IfNode;
use crate::parser::number_node::NumberNode;

#[test]
fn parse_print_node() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];

    let node = parse(&tokens).unwrap();
    assert_eq!(node.children.len(), 1);
    assert_eq!(as_node::<PrintNode>(&node.children[0]), &PrintNode::new("hello"))
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

    assert_eq!(as_node::<NumberNode>(&if_node.left), &NumberNode::new(1111));
    assert_eq!(as_node::<NumberNode>(&if_node.right), &NumberNode::new(2222));
    assert_eq!(as_node::<PrintNode>(&if_node.then), &PrintNode::new("hello"));

//    assert_eq!()
}

#[test]
fn return_error_if_first_token_is_not_command() {
    let tokens = vec![
        Token::string("hello")
    ];

    let result = parse(&tokens);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("Expected command token here".to_string()))

}

fn parse(tokens: &Vec<Token>) -> Result<SequenceNode, String> {
    Parser::new(tokens).parse()
}
