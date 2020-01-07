use crate::tokenizer::token::Token;

type TokenizerResult = Result<Vec<Token>, String>;

pub struct Tokenizer {
    input_chars: Vec<char>,
    result: Vec<Token>,
    position: usize
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input_chars: input.chars().collect(),
            result: Vec::with_capacity(128),
            position: 0
        }
    }

    pub fn tokenize(&mut self) -> TokenizerResult {
        while self.has_input() {
            if self.current_char().is_uppercase() {
                let mut buffer = String::with_capacity(16);
                while self.has_input() && self.current_char().is_uppercase() {
                    buffer.push(self.current_char());
                    self.position += 1;
                }

                self.result.push(Self::keyword_token(&buffer));
            }
            self.position += 1;
        }
        Ok(self.result.clone())
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

    fn has_input(&self) -> bool {
        self.position < self.input_chars.len()
    }

    fn current_char(&self) -> char {
        self.input_chars[self.position]

    }
}