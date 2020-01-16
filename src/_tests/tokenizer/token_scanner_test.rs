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
                (Regex::new("^(PRINT)").unwrap(), Print),
                (Regex::new("^(IF)").unwrap(), If),
                (Regex::new("^(LET)").unwrap(), Let),
                (Regex::new("^(THEN)").unwrap(), Then),
                (Regex::new("^([<>=])").unwrap(), RelOp),
                (Regex::new("^([+-])").unwrap(), TermOp),
                (Regex::new("^([*/])").unwrap(), FactOp),
                (Regex::new("^(^[0-9]+)").unwrap(), Number),
                (Regex::new("^(\".*\")").unwrap(), StringLiteral),
                (Regex::new("^([A-Z])").unwrap(), Var),
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

        return Err("Invalid token".to_string());
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() && self.current_char_is_whitespace().is_whitespace() {
            self.index += 1
        }
    }

    fn current_char_is_whitespace(&mut self) -> char {
        self.input.chars().nth(self.index).unwrap()
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

#[test]
fn scan_relop_tokens() {
    let mut scanner = TokenScanner::new("PRINT < > =");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::relop('<')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('>')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('=')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_termop_and_factop_tokens() {
    let mut scanner = TokenScanner::new("PRINT + - * /");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::termop('+')));
    assert_eq!(scanner.next_token(), Ok(Token::termop('-')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('*')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('/')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_vars() {
    let mut scanner = TokenScanner::new("PRINT A Z");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::var('A')));
    assert_eq!(scanner.next_token(), Ok(Token::var('Z')));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_comma_and_friends() {
    let mut scanner = TokenScanner::new("PRINT , ( )");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::comma()));
    assert_eq!(scanner.next_token(), Ok(Token::openbracket()));
    assert_eq!(scanner.next_token(), Ok(Token::closebracket()));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn handles_invalid_token_in_the_middle_of_valid_ones() {
    let mut scanner = TokenScanner::new("PRINT x 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Err("Invalid token".to_string()));
    assert_eq!(scanner.next_token(), Err("Invalid token".to_string()));
}

#[test]
fn handles_newlines() {
    let mut scanner = TokenScanner::new("\n   PRINT \n\n 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
}

#[test]
fn handles_trailing_whitespace() {
    let mut scanner = TokenScanner::new("PRINT 1234     \n ");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}