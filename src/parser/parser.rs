use crate::parser::ifnode::IfNode;
use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;

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
            root.add(statement);
        }

        Ok(root)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, String> {
        let token = self.consume_token(ANY)?;

        if token.ttype == PRINT {
            let param_token = self.consume_token(STRING)?;
            return Ok(PrintNode::new(param_token.value.as_str()));
        }
        if token.ttype == IF {
            let left_token = self.consume_token(NUMBER)?;
            let relop_token = self.consume_token(RELOP)?;
            let right_token = self.consume_token(NUMBER)?;
            self.consume_token(THEN)?;
            let statement_node = self.parse_statement()?;
            return Ok(IfNode::new(
                NumberNode::new(left_token.value.parse::<i32>().unwrap()),
                NumberNode::new(right_token.value.parse::<i32>().unwrap()),
                relop_token.value.chars().nth(0).unwrap(),
                statement_node))
        } else {
            return Err(format!("Expected command token but got {:?}", token.ttype));
        }
    }

    fn consume_token(&mut self, expected: TokenType) -> Result<Token, String> {
        if self.position == self.tokens.len() {
            return Err(format!("Expected {:?} but reached the end", expected));
        }

        let token = self.tokens[self.position].clone();
        if expected != ANY && token.ttype != expected {
            return Err(format!("Expected {:?} but got {:?}", expected, token.ttype));
        }
        self.position += 1;
        Ok(token)

    }
}