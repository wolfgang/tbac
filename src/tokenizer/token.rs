use TokenType::*;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    PRINT,
    IF,
    THEN,
    GT,
    NUMBER,
    STRING,
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
    pub fn gt() -> Self { Self::with_empty_value(GT) }

    pub fn number(value: &str) -> Self {
        Self { ttype: NUMBER, value: String::from(value) }
    }

    pub fn string(value: &str) -> Self {
        Self { ttype: STRING, value: String::from(value) }
    }

    fn with_empty_value(ttype: TokenType) -> Self {
        Self { ttype, value: String::from("") }
    }
}
