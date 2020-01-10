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

        match token.ttype {
            PRINT => { self.parse_print() }
            IF => { self.parse_if() }
            _ => Err(format!("Expected command token but got {:?}", token.ttype))
        }
    }

    fn parse_print(&mut self) -> Result<Box<dyn Node>, String> {
        let param_token = self.consume_token(STRING)?;
        Ok(PrintNode::new(param_token.value.as_str()))
    }

    fn parse_if(&mut self) -> Result<Box<dyn Node>, String> {
        let left = self.consume_token(NUMBER)?;
        let relop = self.consume_token(RELOP)?;
        let right = self.consume_token(NUMBER)?;
        self.consume_token(THEN)?;
        let statement = self.parse_statement()?;
        Ok(IfNode::new(
            Self::number_node_from(&left),
            Self::number_node_from(&right),
            relop.value.chars().next().unwrap(),
            statement))
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

    fn number_node_from(token: &Token) -> Box<NumberNode> {
        NumberNode::new(token.value.parse::<i32>().unwrap())
    }
}