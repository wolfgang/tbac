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
                (Regex::new("(IF).*").unwrap(), If),
                (Regex::new("(LET).*").unwrap(), Let),
                (Regex::new("(THEN).*").unwrap(), Then),
                (Regex::new("([0-9]+).*").unwrap(), Number),
                (Regex::new("(\".*\").*").unwrap(), StringLiteral),
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
fn scan_uppercase_tokens() {
    let mut scanner = TokenScanner::new("PRINT IF LET THEN");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::iff()));
    assert_eq!(scanner.next_token(), Ok(Token::lett()));
    assert_eq!(scanner.next_token(), Ok(Token::then()));
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
    let mut scanner = TokenScanner::new(r#"PRINT "abcdABCD!""#);
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("abcdABCD!")));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}
