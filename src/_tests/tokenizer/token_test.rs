use crate::tokenizer::token::{Token, TokenType::*};

#[test]
fn can_construct_tokens() {
    let print_token = Token::print();
    let if_token = Token::iff();
    let then_token = Token::then();
    let relop_gt_token = Token::relop('>');
    let number_token = Token::number("1234");
    let string_token = Token::string("abcd");
    let var_token = Token::var('A');

    assert_eq!(PRINT, print_token.ttype);
    assert_eq!(IF, if_token.ttype);
    assert_eq!(THEN, then_token.ttype);

    assert_eq!(RELOP, relop_gt_token.ttype);
    assert_eq!(">", relop_gt_token.value);

    assert_eq!(NUMBER, number_token.ttype);
    assert_eq!(number_token.value, "1234");

    assert_eq!(STRING, string_token.ttype);
    assert_eq!(string_token.value, "abcd");

    assert_eq!(VAR, var_token.ttype);
    assert_eq!(var_token.value, "A");

}