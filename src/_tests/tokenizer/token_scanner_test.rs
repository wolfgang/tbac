use regex::Regex;

use crate::tokenizer::{Token, TokenizerResult};
use crate::tokenizer::token::TokenType::EndOfStream;

struct TokenScanner {
    input: String,
    index: usize,
}

impl TokenScanner {
    pub fn new(input: &str) -> Self {
        Self { input: input.to_string(), index: 0 }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        if self.index == self.input.len() {
            return Ok(Token::end_of_stream());
        }

        while &self.input[self.index..self.index + 1] == " " {
            self.index += 1
        }

        let print_regex = Regex::new("(PRINT).*").unwrap();
        let let_regex = Regex::new("(LET).*").unwrap();

        if print_regex.is_match(&self.input[self.index..]) {
            let caps = print_regex.captures(&self.input[self.index..]).unwrap();
            self.index += caps.get(1).unwrap().as_str().len();
            return Ok(Token::print());
        }

        if let_regex.is_match(&self.input[self.index..]) {
            let caps = let_regex.captures(&self.input[self.index..]).unwrap();
            self.index += caps.get(1).unwrap().as_str().len();
            return Ok(Token::lett());
        }

        return Err("Invalid token".to_string());
    }
}

#[test]
fn returns_print_token_then_end_of_stream() {
    let mut scanner = TokenScanner::new("PRINT");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn returns_first_two_tokens_then_end_of_stream() {
    let mut scanner = TokenScanner::new("PRINT LET");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::lett()));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn regex() {
    let print_regex = Regex::new("(PRINT).*").unwrap();
    let caps = print_regex.captures("PRINT LET").unwrap();
    assert_ne!(caps.len(), 1);
    assert_eq!(caps.get(1).unwrap().as_str(), "PRINT")
}

#[test]
fn string_slicing() {
    let s = "PRINT LET";
    assert_eq!(&s[0..5], "PRINT");
    assert_eq!(&s[5..6], " ");
    assert_ne!(&s[0..1], " ");
    assert_ne!(&s[1..2], " ");
    let s2 = "PRINT";
    assert_ne!(&s2[0..1], " ");
}