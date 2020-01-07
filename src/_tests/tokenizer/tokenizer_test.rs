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
    let input = "PRINT IF THEN GT";
    let result = Tokenizer::new(input).tokenize();

    assert_eq!(result, Ok(vec![
        Token::print(),
        Token::iff(),
        Token::then(),
        Token::gt(),
    ]))

}

#[test]
fn returns_error_if_keyword_is_unknown() {
    let input = "PRINT NOPE IF";
    let result = Tokenizer::new(input).tokenize();
    assert_eq!(result, Err("Unknown keyword 'NOPE'".to_string()));

}