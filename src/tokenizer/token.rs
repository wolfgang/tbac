use TokenType::*;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Print,
    If,
    Then,
    Let,
    RelOp,
    Number,
    StringLiteral,
    Var,
    Comma,
    TermOp,
    FactOp,
    OpenBracket,
    CloseBracket,
    Any,
    EndOfStream
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

impl Token {
    pub fn print() -> Self { Self::with_empty_value(Print) }
    pub fn iff() -> Self { Self::with_empty_value(If) }
    pub fn then() -> Self { Self::with_empty_value(Then) }
    pub fn lett() -> Self { Self::with_empty_value(Let) }
    pub fn relop(op: char) -> Self { Self::with(RelOp, op) }
    pub fn number(value: i32) -> Self { Self::with(Number, value) }
    pub fn string(value: &str) -> Self { Self::with(StringLiteral, value) }
    pub fn var(name: char) -> Self { Self::with(Var, name) }
    pub fn comma() -> Self { Self::with(Comma, ',')}
    pub fn termop(op: char) -> Self {Self::with(TermOp, op)}
    pub fn factop(op: char) -> Self {Self::with(FactOp, op)}
    pub fn openbracket() -> Self { Self::with(OpenBracket, '(')}
    pub fn closebracket() -> Self { Self::with(CloseBracket, ')')}

    pub fn with<T>(ttype: TokenType, value: T) -> Self where T: Display {
        Self { ttype, value: value.to_string() }
    }

    fn with_empty_value(ttype: TokenType) -> Self {
        Self::with(ttype, "")
    }


}
