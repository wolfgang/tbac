use crate::tokenizer::token::{Token, TokenType::*, TokenType};

#[test]
fn can_construct_tokens() {
    assert_token(Token::print(), PRINT, "");
    assert_token(Token::iff(), IF, "");
    assert_token(Token::then(), THEN, "");
    assert_token(Token::lett(), LET, "");

    assert_token(Token::relop('>'), RELOP, ">");

    assert_token(Token::number(1234), NUMBER, "1234");
    assert_token(Token::string("abcd"), STRING, "abcd");

    assert_token(Token::var('A'), VAR, "A");

    assert_token(Token::comma(), COMMA, "");
}

fn assert_token(token: Token, ttype: TokenType, value: &str) {
    assert_eq!(ttype, token.ttype);
    assert_eq!(value, token.value.as_str());
}