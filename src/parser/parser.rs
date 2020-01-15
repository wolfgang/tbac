use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::node::NodeBox;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::string_node::StringNode;
use crate::parser::var_node::VarNode;
use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;
use crate::parser::expression_node::ExpressionNode;

type NodeResult = Result<NodeBox, String>;
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
        let token = self.consume_token(Any)?;

        match token.ttype {
            Print => { self.parse_print() }
            If => { self.parse_if() }
            Let => { self.parse_let() }
            _ => Err(format!("Expected command token but got {:?}", token.ttype))
        }
    }

    fn parse_print(&mut self) -> NodeResult {
        let mut print_node = PrintNode::new();
        loop {
            let token_type = self.peek_token();
            match token_type {
                StringLiteral => {
                    let token = self.consume_token(StringLiteral)?;
                    print_node.add_param(Self::string_node_from(&token))
                }
                _ => { print_node.add_param(self.parse_expression()?) }
            }

            if self.peek_token() != Comma { break; }
            self.consume_token(Comma)?;
        }
        Ok(print_node)
    }

    fn parse_if(&mut self) -> NodeResult {
        let left = self.parse_expression()?;
        let relop = self.consume_token(RelOp)?;
        let right = self.parse_expression()?;
        self.consume_token(Then)?;
        let statement = self.parse_statement()?;
        Ok(IfNode::new(
            left,
            right,
            Self::first_char_of(&relop.value),
            statement))
    }

    fn parse_let(&mut self) -> NodeResult {
        let var_token = self.consume_token(Var)?;
        self.consume_relop("=")?;
        let right_side = self.parse_expression()?;
        Ok(LetNode::new(Self::first_char_of(&var_token.value), right_side))
    }

    fn parse_expression(&mut self) -> NodeResult {
        if self.peek_token() == Number {
            let number_token = self.consume_token(Number)?;

            if self.peek_token() == TermOp {
                let op_token = self.consume_token(TermOp)?;
                let right = self.parse_expression()?;
                return Ok(ExpressionNode::new(
                    Self::first_char_of(&op_token.value),
                    Self::number_node_from(&number_token),
                    right
                ));


            }

            return Ok(Self::number_node_from(&number_token));
        }

        let token = self.consume_token(Var)?;
        Ok(Self::var_node_from(&token))
    }

    fn consume_relop(&mut self, expected: &str) -> TokenResult {
        let relop = self.consume_token(RelOp)?;
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
        if expected != Any && token.ttype != expected {
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
        VarNode::new(Self::first_char_of(&token.value))
    }

    fn first_char_of(s: &String) -> char {
        s.chars().next().unwrap()
    }
}