use crate::tokenizer::token::{Token, TokenType::*, TokenType};

#[test]
fn can_construct_tokens() {
    assert_token(Token::print(), Print, "");
    assert_token(Token::iff(), If, "");
    assert_token(Token::then(), Then, "");
    assert_token(Token::lett(), Let, "");

    assert_token(Token::bin_op('>'), BinOp, ">");

    assert_token(Token::number(1234), Number, "1234");
    assert_token(Token::string("abcd"), StringLiteral, "abcd");

    assert_token(Token::var('A'), Var, "A");

    assert_token(Token::comma(), Comma, "");
}

fn assert_token(token: Token, ttype: TokenType, value: &str) {
    assert_eq!(ttype, token.ttype);
    assert_eq!(value, token.value.as_str());
}