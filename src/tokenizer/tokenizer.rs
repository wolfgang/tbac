use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;
use regex::Regex;
use crate::tokenizer::Token;

pub type TokenizerResult = Result<Vec<Token>, String>;

pub fn tokenize(input: &str) -> TokenizerResult {
    Tokenizer::new(input).tokenize()
}

pub struct Tokenizer<'a> {
    input: &'a str,
    index: usize,
    token_matchers: Vec<(Regex, TokenType)>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            token_matchers: vec![
                (Regex::new("^(PRINT)").unwrap(), Print),
                (Regex::new("^(IF)").unwrap(), If),
                (Regex::new("^(LET)").unwrap(), Let),
                (Regex::new("^(GOTO)").unwrap(), Goto),
                (Regex::new("^(THEN)").unwrap(), Then),
                (Regex::new("^([<>=])").unwrap(), RelOp),
                (Regex::new("^([+-])").unwrap(), TermOp),
                (Regex::new("^([*/])").unwrap(), FactOp),
                (Regex::new("^(^[0-9]+)").unwrap(), Number),
                (Regex::new("^(\"[A-Za-z0-9<>=:+\\-\\s]+\")").unwrap(), StringLiteral),
                (Regex::new("^([A-Z])(?:[^A-Z]|$)").unwrap(), Var),
                (Regex::new("^(,)").unwrap(), Comma),
                (Regex::new("^(\\()").unwrap(), OpenBracket),
                (Regex::new("^(\\))").unwrap(), CloseBracket),
            ],
        }
    }

    pub fn tokenize(&mut self) -> TokenizerResult {
        let mut result = Vec::with_capacity(128);
        loop {
            match self.next_token() {
                Err(e) => { return Err(e); }
                Ok(token) => {
                    if token.ttype == EndOfStream { return Ok(result); }
                    result.push(token);
                }
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        if self.index == self.input.len() {
            return Ok(Token::end_of_stream());
        }

        let slice = &self.input[self.index..];
        for matcher in &self.token_matchers {
            let (regex, token_type) = matcher;
            if regex.is_match(slice) {
                let value = Self::get_token_value(regex, slice);
                self.index += value.len();
                return Ok(Self::token_with(*token_type, value));
            }
        }

        return Err(format!("Invalid token at '{}'", slice));
    }

    fn get_token_value<'b>(regex: &Regex, slice: &'b str) -> &'b str {
        let captures = regex.captures(slice).unwrap();
        captures.get(1).unwrap().as_str()
    }

    fn token_with(ttype: TokenType, value: &str) -> Token {
        let value = if ttype == StringLiteral { &value[1..value.len() - 1] } else { value };
        Token::with(ttype, value)
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