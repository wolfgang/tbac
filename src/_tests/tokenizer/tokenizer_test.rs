use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

#[test]
fn tokenizes_print_statement_without_parameters() {
    let input = r#"PRINT"#;
    let result = Tokenizer::new(input).tokenize();

    assert_eq!(result, Ok(vec![
        Token::print()
    ]))

}

#[test]
fn tokenizes_all_keywords() {
    let input = r#"PRINT IF THEN GT"#;
    let result = Tokenizer::new(input).tokenize();

    assert_eq!(result, Ok(vec![
        Token::print(),
        Token::iff(),
        Token::then(),
        Token::gt(),
    ]))

}