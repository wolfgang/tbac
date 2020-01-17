use crate::tokenizer::token::{Token, TokenType::*, TokenType};

#[test]
fn can_construct_tokens() {
    assert_token(Token::statement("PRINT"), Statement, "PRINT");


    assert_token(Token::print(), Statement, "PRINT");
    assert_token(Token::iff(), Statement, "IF");
    assert_token(Token::lett(), Statement, "LET");
    assert_token(Token::goto(), Statement, "GOTO");

    assert_token(Token::then(), Then, "THEN");

    assert_token(Token::relop('>'), RelOp, ">");

    assert_token(Token::number(1234), Number, "1234");
    assert_token(Token::string("abcd"), StringLiteral, "abcd");

    assert_token(Token::var('A'), Var, "A");

    assert_token(Token::comma(), Comma, ",");

    assert_token(Token::termop('+'), TermOp, "+");
    assert_token(Token::factop('*'), FactOp, "*");

    assert_token(Token::openbracket(), OpenBracket, "(");
    assert_token(Token::closebracket(), CloseBracket, ")");
}

fn assert_token(token: Token, ttype: TokenType, value: &str) {
    assert_eq!(ttype, token.ttype);
    assert_eq!(value, token.value.as_str());
}