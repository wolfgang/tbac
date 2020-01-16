use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;
use regex::Regex;

pub struct Tokenizer {
    input: String,
    index: usize,
    token_matchers: Vec<(Regex, TokenType)>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            index: 0,
            token_matchers: vec![
                (Regex::new("^(PRINT)").unwrap(), Print),
                (Regex::new("^(IF)").unwrap(), If),
                (Regex::new("^(LET)").unwrap(), Let),
                (Regex::new("^(THEN)").unwrap(), Then),
                (Regex::new("^([<>=])").unwrap(), RelOp),
                (Regex::new("^([+-])").unwrap(), TermOp),
                (Regex::new("^([*/])").unwrap(), FactOp),
                (Regex::new("^(^[0-9]+)").unwrap(), Number),
                (Regex::new("^(\"[A-Za-z0-9]+\")").unwrap(), StringLiteral),
                (Regex::new(r"^([A-Z])(?:[^A-Z]|$)").unwrap(), Var),
                (Regex::new("^(,)").unwrap(), Comma),
                (Regex::new("^(\\()").unwrap(), OpenBracket),
                (Regex::new("^(\\))").unwrap(), CloseBracket),
            ],
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        if self.index == self.input.len() {
            return Ok(Token::end_of_stream());
        }

        for matcher in &self.token_matchers {
            let (regex, token_type) = matcher;
            if regex.is_match(&self.input[self.index..]) {
                let caps = regex.captures(&self.input[self.index..]).unwrap();
                let captured = caps.get(1).unwrap().as_str();
                self.index += captured.len();
                let value = if *token_type == StringLiteral { &captured[1..captured.len() - 1] } else { captured };
                return Ok(Token::with(*token_type, value));
            }
        }

        return Err(format!("Invalid token at '{}'", self.input[self.index..].to_string()));
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() && self.current_char_is_whitespace() {
            self.index += 1
        }
    }

    fn current_char_is_whitespace(&self) -> bool {
        self.input.chars().nth(self.index).unwrap().is_whitespace()
    }
}