use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::PRINT;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self { tokens: tokens.clone(), position: 0 }
    }

    pub fn parse(&mut self) -> SequenceNode {
        let mut root = SequenceNode::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            if token.ttype == PRINT {
                self.position += 1;
                let param_token = &self.tokens[self.position];
                root.add(PrintNode::new(param_token.value.as_str()))
            }

            self.position += 1;
        }

        root
    }
}