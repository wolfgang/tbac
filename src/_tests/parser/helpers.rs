use crate::tokenizer::Token;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::parser::Parser;

pub fn parse(tokens: &Vec<Token>) -> Result<SequenceNode, String> {
    Parser::new(tokens).parse()
}

pub fn assert_parse_error(result: Result<SequenceNode, String>, error: &str) {
    assert!(result.is_err());
    assert_eq!(result.err(), Some(error.to_string()))
}