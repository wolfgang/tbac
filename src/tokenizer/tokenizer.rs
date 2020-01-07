use crate::tokenizer::token::Token;

type TokenizerResult = Result<Vec<Token>, String>;

pub struct Tokenizer {
    input_chars: Vec<char>
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self { input_chars: input.chars().collect() }
    }

    pub fn tokenize(&self) -> TokenizerResult {
        let mut position = 0;
        let mut result = Vec::with_capacity(128);
        while position < self.input_chars.len() {
            if self.input_chars[position].is_uppercase() {
                let mut buffer = String::with_capacity(16);
                while position < self.input_chars.len() && self.input_chars[position].is_uppercase() {
                    buffer.push(self.input_chars[position]);
                    position += 1;
                }

                if buffer.as_str() == "PRINT" { result.push(Token::print()) }
                if buffer.as_str() == "IF" { result.push(Token::iff())};
                if buffer.as_str() == "THEN" { result.push(Token::then())};
                if buffer.as_str() == "GT" { result.push(Token::gt())};
            }
            position += 1;
        }
        Ok(result)
    }
}