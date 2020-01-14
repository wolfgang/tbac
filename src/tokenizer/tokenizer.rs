use crate::tokenizer::token::Token;

pub type TokenizerResult = Result<Vec<Token>, String>;

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
            self.skip_whitespace();
            if self.current_char_is(|c| c.is_uppercase()) {
                self.read_uppercase_str()?;
                continue;
            }
            if self.current_char_is_char('"') {
                self.read_string()?;
                continue;
            }
            if self.current_char_is(|c| c.is_digit(10)) {
                self.read_number();
                continue;
            }
            if self.current_char_is_binop() {
                self.read_binop();
                continue;
            }
            if self.current_char_is_char(',') {
                self.consume_char();
                self.result.push(Token::comma());
                continue;
            }

            if self.has_input() {
                return Err(format!("Unrecognized character '{}'", self.current_char()));
            }
        }
        Ok(self.result.clone())
    }

    fn skip_whitespace(&mut self) {
        self.consume_chars_while(|c| c.is_whitespace());
    }

    fn read_uppercase_str(&mut self) -> Result<(), String> {
        let buffer = self.consume_chars_while(|c| c.is_uppercase());

        match Self::token_from_uppercase_str(&buffer) {
            Some(token) => {
                self.result.push(token);
                Ok(())
            }
            None => { Err(format!("Unknown keyword '{}'", buffer)) }
        }
    }

    fn read_string(&mut self) -> Result<(), String> {
        self.consume_char();
        let buffer = self.consume_chars_while(|c| c != '"');
        if !self.has_input() {
            return Err(format!("Unterminated string '{}'", buffer));
        }
        self.consume_char();
        self.result.push(Token::string(buffer.as_str()));
        Ok(())
    }

    fn read_number(&mut self) {
        let buffer = self.consume_chars_while(|c| c.is_digit(10));
        self.result.push(Token::number(buffer.parse().unwrap()))
    }

    fn current_char_is_binop(&self) -> bool {
        self.current_char_is(|c| c == '>' || c == '<' || c == '=')
    }

    fn read_binop(&mut self) {
        self.result.push(Token::bin_op(self.current_char()));
        self.consume_char();
    }

    pub fn token_from_uppercase_str(name: &String) -> Option<Token> {
        if name.len() == 1 {
            return Some(Token::var(name.chars().next().unwrap()));
        }

        match name.as_str() {
            "PRINT" => { Some(Token::print()) }
            "IF" => { Some(Token::iff()) }
            "THEN" => { Some(Token::then()) }
            "LET" => { Some(Token::lett()) }
            _ => { None }
        }
    }


    fn consume_chars_while<F>(&mut self, pred: F) -> String where F: Fn(char) -> bool {
        let mut buffer = String::with_capacity(128);
        while self.has_input() && pred(self.current_char()) {
            buffer.push(self.consume_char());
        }

        buffer
    }

    fn current_char_is_char(&self, expected: char) -> bool {
        self.current_char_is(|c| c == expected)
    }

    fn current_char_is<F>(&self, pred: F) -> bool where F: Fn(char) -> bool {
        self.has_input() && pred(self.current_char())
    }

    fn has_input(&self) -> bool {
        self.position < self.input_chars.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.current_char();
        self.position += 1;
        c
    }

    fn current_char(&self) -> char {
        self.input_chars[self.position]
    }
}