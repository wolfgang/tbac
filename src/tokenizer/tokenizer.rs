use crate::tokenizer::token::Token;

type TokenizerResult = Result<Vec<Token>, String>;

pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new(_input: &str) -> Self {
        Self {}
    }

    pub fn tokenize(&self) -> TokenizerResult {
        Ok(Vec::new())
    }
}