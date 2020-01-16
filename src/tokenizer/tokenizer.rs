use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenType::EndOfStream;
use crate::tokenizer::token_scanner::TokenScanner;

pub type TokenizerResult = Result<Vec<Token>, String>;

pub fn tokenize(input: &str) -> TokenizerResult {
    let mut scanner = TokenScanner::new(input);
    let mut result = Vec::with_capacity(128);
    loop {
        match scanner.next_token() {
            Err(e) => { return Err(e); }
            Ok(token) => {
                if token.ttype == EndOfStream { return Ok(result); }
                result.push(token);
            }
        }
    }
}