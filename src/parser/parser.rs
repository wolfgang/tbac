use crate::tokenizer::Token;
use crate::parser::sequence_node::SequenceNode;

pub struct Parser {

}

impl Parser {
    pub fn new(_tokens: &Vec<Token>) -> Self {
        Self {}
    }

    pub fn parse(&mut self) -> SequenceNode {
        SequenceNode::new()
    }
}