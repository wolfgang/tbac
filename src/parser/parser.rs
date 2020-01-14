use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::string_node::StringNode;
use crate::parser::var_node::VarNode;
use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;

type NodeResult = Result<Box<dyn Node>, String>;
type TokenResult = Result<Token, String>;

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

    fn parse_statement(&mut self) -> NodeResult {
        let token = self.consume_token(ANY)?;

        match token.ttype {
            PRINT => { self.parse_print() }
            IF => { self.parse_if() }
            LET => { self.parse_let() }
            _ => Err(format!("Expected command token but got {:?}", token.ttype))
        }
    }

    fn parse_print(&mut self) -> NodeResult {
        let mut print_node = PrintNode::new();
        loop {
            let token = self.consume_token(ANY)?;
            match token.ttype {
                STRING => {
                    print_node.add_param(Self::string_node_from(&token))
                }
                NUMBER => {
                    print_node.add_param(Self::number_node_from(&token))
                }
                _ => {
                    return Err("Expected STRING or NUMBER but got THEN".to_string())
                }
            }

            if self.peek_token() != COMMA { break; }
            self.consume_token(COMMA)?;
        }
        Ok(print_node)
    }

    fn parse_if(&mut self) -> NodeResult {
        let left = self.parse_expression()?;
        let relop = self.consume_token(RELOP)?;
        let right = self.parse_expression()?;
        self.consume_token(THEN)?;
        let statement = self.parse_statement()?;
        Ok(IfNode::new(
            left,
            right,
            relop.value.chars().next().unwrap(),
            statement))
    }

    fn parse_let(&mut self) -> NodeResult {
        let var = self.consume_token(VAR)?;
        self.consume_relop("=")?;
        let value = self.consume_token(NUMBER)?;
        Ok(LetNode::new(var.value.chars().next().unwrap(), Self::number_node_from(&value)))
    }

    fn parse_expression(&mut self) -> NodeResult {
        if self.peek_token() == NUMBER {
            let token = self.consume_token(NUMBER)?;
            return Ok(Self::number_node_from(&token));
        }

        let token = self.consume_token(VAR)?;
        Ok(Self::var_node_from(&token))
    }

    fn consume_relop(&mut self, expected: &str) -> TokenResult {
        let relop = self.consume_token(RELOP)?;
        if relop.value.as_str() != expected {
            return Err(format!("Expected {} but got {}", expected, relop.value));
        }
        Ok(relop)
    }

    fn consume_token(&mut self, expected: TokenType) -> TokenResult {
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

    fn peek_token(&self) -> TokenType {
        if self.position == self.tokens.len() {
            return EndOfStream;
        }
        self.tokens[self.position].ttype
    }

    fn number_node_from(token: &Token) -> Box<NumberNode> {
        NumberNode::new(token.value.parse::<i32>().unwrap())
    }

    fn string_node_from(token: &Token) -> Box<StringNode> {
        StringNode::new(token.value.as_str())
    }

    fn var_node_from(token: &Token) -> Box<VarNode> {
        VarNode::new(token.value.chars().next().unwrap())
    }
}