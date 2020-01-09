use crate::tokenizer::Token;
use crate::parser::parser::Parser;
use crate::_tests::helpers::as_node;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;

#[test]
fn parse_print_node() {
    let tokens = vec![
        Token::print(),
        Token::string("hello")
    ];

    let node = parse(&tokens);
    assert_eq!(node.children.len(), 1);
    assert_eq!(as_node::<PrintNode>(&node.children[0]), &PrintNode::new("hello"))
}

fn parse(tokens: &Vec<Token>) -> SequenceNode {
    Parser::new(tokens).parse()
}
