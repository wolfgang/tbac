use crate::tokenizer::{Tokenizer, TokenizerResult};
use crate::tokenizer::Token;

fn tokenize(input: &str) -> TokenizerResult {
    Tokenizer::new(input).tokenize()
}

#[test]
fn tokenizes_print_statement_without_parameters() {
    assert_eq!(tokenize("PRINT"),
               Ok(vec![Token::print()]))
}

#[test]
fn tokenizes_all_keywords() {
    assert_eq!(tokenize("PRINT IF THEN"),
               Ok(vec![
                   Token::print(),
                   Token::iff(),
                   Token::then()
               ]))
}

#[test]
fn tokenizes_string_literals() {
    assert_eq!(tokenize(r#"PRINT "hello" PRINT "world""#),
               Ok(vec![
                   Token::print(),
                   Token::string("hello"),
                   Token::print(),
                   Token::string("world"),
               ]))
}

#[test]
fn tokenizes_numbers() {
    assert_eq!(tokenize("PRINT 12345"),
               Ok(vec![
                   Token::print(),
                   Token::number("12345")
               ]))
}

#[test]
fn tokenize_gt() {
    assert_eq!(tokenize("12 > 10"),
               Ok(vec![
                   Token::number("12"),
                   Token::relop('>'),
                   Token::number("10")
               ])
    )
}

#[test]
fn returns_error_if_keyword_is_unknown() {
    assert_eq!(tokenize("PRINT NOPE"),
               Err("Unknown keyword 'NOPE'".to_string()));
}

#[test]
fn returns_error_if_string_is_unterminated() {
    assert_eq!(tokenize(r#"PRINT "hello" PRINT "hellooo"#),
               Err("Unterminated string 'hellooo'".to_string()));
}

#[test]
fn returns_error_if_encountering_unknown_token() {
    assert_eq!(tokenize("PRINT 123 %"),
               Err("Unrecognized character '%'".to_string()));
}