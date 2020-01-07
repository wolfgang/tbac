use TokenType::*;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    PRINT,
    IF,
    THEN,
    GT,
    NUMBER,
    STRING,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

impl Token {
    pub fn print() -> Self { Self::keyword(PRINT) }
    pub fn iff() -> Self { Self::keyword(IF) }
    pub fn then() -> Self { Self::keyword(THEN) }
    pub fn gt() -> Self { Self::keyword(GT) }

    pub fn number(value: &str) -> Self {
        Self { ttype: NUMBER, value: String::from(value) }
    }

    pub fn string(value: &str) -> Self {
        Self { ttype: STRING, value: String::from(value) }
    }

    fn keyword(ttype: TokenType) -> Self {
        Self { ttype, value: String::from("") }
    }
}
