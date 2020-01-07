use crate::tokenizer::token::Token;

type TokenizerResult = Result<Vec<Token>, String>;

pub struct Tokenizer {
    input_chars: Vec<char>,
    result: Vec<Token>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input_chars: input.chars().collect(),
            result: Vec::with_capacity(128),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> TokenizerResult {
        while self.has_input() {
            if self.current_char().is_uppercase() { self.read_keyword()? }
            if self.has_input() && self.current_char() == '"' { self.read_string() ? }
            self.position += 1;
        }
        Ok(self.result.clone())
    }

    fn read_keyword(&mut self) -> Result<(), String> {
        let mut buffer = String::with_capacity(16);
        while self.has_input() && self.current_char().is_uppercase() {
            buffer.push(self.current_char());
            self.position += 1;
        }

        match Self::keyword_token(&buffer) {
            Some(token) => {
                self.result.push(token);
                Ok(())
            }
            None => { Err(format!("Unknown keyword '{}'", buffer)) }
        }
    }

    fn read_string(&mut self) -> Result<(), String> {
        self.position += 1;
        let mut buffer = String::with_capacity(128);
        while self.has_input() && self.current_char() != '"' {
            buffer.push(self.current_char());
            self.position += 1;
        }

        self.result.push(Token::string(buffer.as_str()));

        return Ok(())

    }

    fn keyword_token(buffer: &String) -> Option<Token> {
        match buffer.as_str() {
            "PRINT" => { Some(Token::print()) }
            "IF" => { Some(Token::iff()) }
            "THEN" => { Some(Token::then()) }
            "GT" => { Some(Token::gt()) }
            _ => { None }
        }
    }

    fn has_input(&self) -> bool {
        self.position < self.input_chars.len()
    }

    fn current_char(&self) -> char {
        self.input_chars[self.position]
    }
}