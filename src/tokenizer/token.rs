use TokenType::*;

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
    pub fn relop(op: char) -> Self { Self::with_char(RELOP, op) }
    pub fn number(value: &str) -> Self { Self::with_str(NUMBER, value) }
    pub fn string(value: &str) -> Self { Self::with_str(STRING, value) }
    pub fn var(name: char) -> Self { Self::with_char(VAR, name) }

    fn with_empty_value(ttype: TokenType) -> Self {
        Self::with_str(ttype, "")
    }

    fn with_str(ttype: TokenType, value: &str) -> Self {
        Self { ttype, value: value.to_string() }
    }

    fn with_char(ttype: TokenType, value: char) -> Self {
        Self { ttype, value: format!("{}", value) }
    }
}
