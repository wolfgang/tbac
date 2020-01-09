use crate::parser::ifnode::IfNode;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::{IF, PRINT};
use crate::parser::node::Node;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self { tokens: tokens.clone(), position: 0 }
    }

    pub fn parse(&mut self) -> Result<SequenceNode, String> {
        let mut root = SequenceNode::new();

        while self.position < self.tokens.len() {
            let statement = self.parse_statement()?;
            root.add2(statement);
        }

        Ok(root)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, String> {
        let token = &self.tokens[self.position];
        if token.ttype == PRINT {
            self.position += 1;
            let param_token = &self.tokens[self.position];
            self.position += 1;
            return Ok(Box::new(PrintNode::new(param_token.value.as_str())));
        }
        if token.ttype == IF {
            self.position += 1;
            let left_token = self.consume_token();
            let relop_token = self.consume_token();
            let right_token = self.consume_token();
            let _then_token = self.consume_token();
            let statement_node : Box<dyn Node> = self.parse_statement().unwrap();
            return Ok(Box::new(IfNode::new3(
                Box::new(NumberNode::new(left_token.value.parse::<i32>().unwrap())),
                Box::new(NumberNode::new(right_token.value.parse::<i32>().unwrap())),
                relop_token.value.chars().nth(0).unwrap(),
                statement_node)))
        } else {
            return Err("Expected command token here".to_string());
        }
    }

    fn consume_token(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token

    }
}