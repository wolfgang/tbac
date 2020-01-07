use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

#[test]
fn tokenizes_print_statement() {
    let input = r#"PRINT "HELLO""#;
    let result = Tokenizer::new(input).tokenize();

    assert_eq!(result, Ok(vec![
        Token::print(),
        Token::string("HELLO")
    ]))

}