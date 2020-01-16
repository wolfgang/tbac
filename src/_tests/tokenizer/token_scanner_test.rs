use regex::Regex;

use crate::tokenizer::Token;
use crate::tokenizer::token::TokenType::*;
use crate::tokenizer::token::TokenType;

struct TokenScanner {
    input: String,
    index: usize,
    token_matchers: Vec<(Regex, TokenType)>,
}

impl TokenScanner {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            index: 0,
            token_matchers: vec![
                (Regex::new("(PRINT).*").unwrap(), Print),
                (Regex::new("(LET).*").unwrap(), Let),
                (Regex::new("([0-9]+).*").unwrap(), Number),
                (Regex::new("(\"[a-z]+\").*").unwrap(), StringLiteral),
            ],
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        if self.index == self.input.len() {
            return Ok(Token::end_of_stream());
        }

        while &self.input[self.index..self.index + 1] == " " {
            self.index += 1
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
fn scan_number_tokens() {
    let mut scanner = TokenScanner::new("PRINT 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_string_tokens() {
    let mut scanner = TokenScanner::new(r#"PRINT "abcd""#);
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("abcd")));
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