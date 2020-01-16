use std::fmt::Display;

use TokenType::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Print,
    If,
    Let,
    Goto,
    Then,
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
    EndOfStream,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

impl Token {
    pub fn print() -> Self { Self::with(Print, "PRINT") }
    pub fn iff() -> Self { Self::with(If, "IF") }
    pub fn lett() -> Self { Self::with(Let, "LET") }
    pub fn goto() -> Self { Self::with(Goto, "GOTO") }
    pub fn then() -> Self { Self::with(Then, "THEN") }
    pub fn relop(op: char) -> Self { Self::with(RelOp, op) }
    pub fn number(value: i32) -> Self { Self::with(Number, value) }
    pub fn string(value: &str) -> Self { Self::with(StringLiteral, value) }
    pub fn var(name: char) -> Self { Self::with(Var, name) }
    pub fn comma() -> Self { Self::with(Comma, ',') }
    pub fn termop(op: char) -> Self { Self::with(TermOp, op) }
    pub fn factop(op: char) -> Self { Self::with(FactOp, op) }
    pub fn openbracket() -> Self { Self::with(OpenBracket, '(') }
    pub fn closebracket() -> Self { Self::with(CloseBracket, ')') }
    pub fn end_of_stream() -> Self { Self::with_empty_value(EndOfStream) }

    pub fn with<T>(ttype: TokenType, value: T) -> Self where T: Display {
        Self { ttype, value: value.to_string() }
    }

    pub fn value_as_char(&self) -> char {
        self.value.chars().next().unwrap()
    }

    pub fn value_as_int(&self) -> i32 {
        self.value.parse::<i32>().unwrap()
    }

    pub fn value_as_str(&self) -> &str {
        self.value.as_str()
    }

    fn with_empty_value(ttype: TokenType) -> Self {
        Self::with(ttype, "")
    }
}
