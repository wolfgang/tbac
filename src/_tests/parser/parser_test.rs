use crate::tokenizer::Token;
use crate::parser::parser::Parser;

#[test]
fn return_empty_sequence_node_for_empty_token_list() {
    let tokens: Vec<Token>= Vec::new();
    let mut parser = Parser::new(&tokens);
    let sequence_node = parser.parse();

    assert_eq!(sequence_node.children.len(), 0);
}