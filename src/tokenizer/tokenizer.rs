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

                result.push(Self::keyword_token(&buffer));
            }
            position += 1;
        }
        Ok(result)
    }

    fn keyword_token(buffer: &String) -> Token {
        match buffer.as_str() {
            "PRINT" => { Token::print() }
            "IF" => { Token::iff() }
            "THEN" => { Token::then() }
            "GT" => { Token::gt() }
            _ => { Token::number("1234") }
        }
    }
}