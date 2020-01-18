use crate::parser::expression_node::ExpressionNode;
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
use crate::parser::goto_node::GotoNode;

type NodeResult = Result<NodeBox, String>;
type TokenResult = Result<Token, String>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<SequenceNode, String> {
        let mut root = SequenceNode::new();

        while self.peek_token() != EndOfStream {
            let statement = self.parse_statement()?;
            root.add(statement);
        }
        Ok(root)
    }

    fn parse_statement(&mut self) -> NodeResult {
        let mut token = self.consume_token(Any)?;
        let mut line = 0;
        if token.ttype == Number {
            line = token.value_as_uint();
            token = self.consume_token(Statement)?;
        }

        if token.ttype != Statement {
            return Err(format!("Expected command token but got {:?}", token.ttype));
        }

        match token.value.as_str() {
            "PRINT" => { self.parse_print(line) }
            "IF" => { self.parse_if(line) }
            "LET" => { self.parse_let(line) }
            "GOTO" => { self.parse_goto(line) }
            _ => Err(format!("Invalid statement '{}'", token.value))
        }
    }

    fn parse_print(&mut self, line: u32) -> NodeResult {
        let mut print_node = PrintNode::new(line);
        loop {
            match self.peek_token() {
                StringLiteral => {
                    let token = self.consume_token(StringLiteral)?;
                    print_node.add_param(StringNode::new(token.value_as_str()))
                }
                _ => {
                    print_node.add_param(self.parse_expression(false)?)
                }
            }

            if self.peek_token() != Comma { break; }
            self.consume_token(Comma)?;
        }
        Ok(print_node)
    }

    fn parse_if(&mut self, line: u32) -> NodeResult {
        let left = self.parse_expression(false)?;
        let relop = self.consume_token(RelOp)?;
        let right = self.parse_expression(false)?;
        self.consume_token(Then)?;
        let statement = self.parse_statement()?;
        Ok(IfNode::new(left, right, relop.value_as_char(), statement, line))
    }

    fn parse_let(&mut self, line: u32) -> NodeResult {
        let token = self.consume_token(Var)?;
        self.consume_equal_sign()?;
        let right = self.parse_expression(false)?;
        Ok(LetNode::new(token.value_as_char(), right, line))
    }

    fn parse_goto(&mut self, line: u32) -> NodeResult {
        let destination = self.parse_expression(false)?;
        Ok(GotoNode::new(destination, line))
    }

    fn parse_expression(&mut self, in_brackets: bool) -> NodeResult {
        let left = self.parse_term()?;
        if self.peek_token() == TermOp {
            let op_token = self.consume_token(TermOp)?;
            let right = self.parse_expression(false)?;
            return Ok(ExpressionNode::new(op_token.value_as_char(), in_brackets, left, right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> NodeResult {
        let left = self.parse_factor()?;
        if self.peek_token() == FactOp {
            let op_token = self.consume_token(FactOp)?;
            let right = self.parse_term()?;
            return Ok(ExpressionNode::new(op_token.value_as_char(), false, left, right));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> NodeResult {
        match self.peek_token() {
            OpenBracket => {
                self.consume_token(OpenBracket)?;
                let result = self.parse_expression(true);
                self.consume_token(CloseBracket)?;
                result
            }
            Number => {
                let token = self.consume_token(Number)?;
                Ok(NumberNode::new(token.value_as_int()))
            }

            _ => {
                let token = self.consume_token(Var)?;
                Ok(VarNode::new(token.value_as_char()))
            }
        }
    }

    fn consume_equal_sign(&mut self) -> TokenResult {
        let relop = self.consume_token(RelOp)?;
        if relop.value.as_str() != "=" {
            return Err(format!("Expected = but got {}", relop.value));
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
}
