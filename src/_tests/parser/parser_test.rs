use crate::tokenizer::Token;
use crate::parser::parser::Parser;
use crate::_tests::helpers::as_node;
use crate::parser::print_node::PrintNode;

#[test]
fn parse_print_node() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];

    let mut parser = Parser::new(&tokens);
    let node = parser.parse();
    assert_eq!(node.children.len(), 1);
    assert_eq!(as_node::<PrintNode>(&node.children[0]), &PrintNode::new("hello"))
}