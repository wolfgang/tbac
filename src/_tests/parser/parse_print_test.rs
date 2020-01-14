use crate::_tests::helpers::assert_print_node;
use crate::tokenizer::Token;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::parser::Parser;

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

fn parse(tokens: &Vec<Token>) -> Result<SequenceNode, String> {
    Parser::new(tokens).parse()
}
