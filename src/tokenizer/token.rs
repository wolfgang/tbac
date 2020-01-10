use TokenType::*;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    PRINT,
    IF,
    THEN,
    RELOP,
    NUMBER,
    STRING,
    VAR,
    ANY,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

impl Token {
    pub fn print() -> Self { Self::with_empty_value(PRINT) }
    pub fn iff() -> Self { Self::with_empty_value(IF) }
    pub fn then() -> Self { Self::with_empty_value(THEN) }
    pub fn relop(op: char) -> Self { Self::with(RELOP, op) }
    pub fn number(value: i32) -> Self { Self::with(NUMBER, value) }
    pub fn string(value: &str) -> Self { Self::with(STRING, value) }
    pub fn var(name: char) -> Self { Self::with(VAR, name) }

    fn with_empty_value(ttype: TokenType) -> Self {
        Self::with(ttype, "")
    }

    fn with<T>(ttype: TokenType, value: T) -> Self where T: Display {
        Self { ttype, value: value.to_string() }
    }


}
