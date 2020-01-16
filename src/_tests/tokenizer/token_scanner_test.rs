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
        if &self.input[self.index..5] == "PRINT" {
            self.index += 5;
            return Ok(Token::print());
        }

        return Ok(Token::openbracket())
    }
}

#[test]
fn returns_print_token_then_end_of_string() {
    let mut scanner = TokenScanner::new("PRINT");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}