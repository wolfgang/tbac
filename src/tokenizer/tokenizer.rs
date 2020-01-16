use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenType::EndOfStream;
use crate::tokenizer::token_scanner::Tokenizer;

pub type TokenizerResult = Result<Vec<Token>, String>;

pub fn tokenize(input: &str) -> TokenizerResult {
    Tokenizer::new(input).tokenize()
}