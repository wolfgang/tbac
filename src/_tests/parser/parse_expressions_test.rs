use crate::tokenizer::Token;

#[test]
fn parses_binary_expressions_in_let() {
    let _tokens = vec![
        Token::lett(),
        Token::var('A'),
        Token::relop('='),
        Token::number(10),
        Token::termop('+'),
        Token::number(20)
    ];

    assert!(false, "Expecting LET with expression node")


}