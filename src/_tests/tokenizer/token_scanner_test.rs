
use crate::tokenizer::Token;
use crate::tokenizer::token_scanner::Tokenizer;


#[test]
fn scan_uppercase_tokens() {
    let mut scanner = Tokenizer::new("PRINT IF LET THEN");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::iff()));
    assert_eq!(scanner.next_token(), Ok(Token::lett()));
    assert_eq!(scanner.next_token(), Ok(Token::then()));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_number_tokens() {
    let mut scanner = Tokenizer::new("PRINT 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_string_tokens() {
    let mut scanner = Tokenizer::new(r#"PRINT "abcdABCD1234""#);
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("abcdABCD1234")));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_relop_tokens() {
    let mut scanner = Tokenizer::new("PRINT < > =");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::relop('<')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('>')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('=')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_termop_and_factop_tokens() {
    let mut scanner = Tokenizer::new("PRINT + - * /");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::termop('+')));
    assert_eq!(scanner.next_token(), Ok(Token::termop('-')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('*')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('/')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_vars() {
    let mut scanner = Tokenizer::new("PRINT A, Z");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::var('A')));
    assert_eq!(scanner.next_token(), Ok(Token::comma()));
    assert_eq!(scanner.next_token(), Ok(Token::var('Z')));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_comma_and_friends() {
    let mut scanner = Tokenizer::new("PRINT , ( )");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::comma()));
    assert_eq!(scanner.next_token(), Ok(Token::openbracket()));
    assert_eq!(scanner.next_token(), Ok(Token::closebracket()));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn handles_invalid_token_in_the_middle_of_valid_ones() {
    let mut scanner = Tokenizer::new("PRINT x 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Err("Invalid token at 'x 1234'".to_string()));
    assert_eq!(scanner.next_token(), Err("Invalid token at 'x 1234'".to_string()));
}

#[test]
fn handles_newlines() {
    let mut scanner = Tokenizer::new("\n   PRINT \n\n 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
}

#[test]
fn handles_trailing_whitespace() {
    let mut scanner = Tokenizer::new("PRINT 1234     \n ");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_multiple_tokens_of_same_type() {
    let mut scanner = Tokenizer::new(r#"PRINT "hello" PRINT "world""#);
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("hello")));
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("world")));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn return_error_or_invalid_uppercase_word() {
    let mut scanner = Tokenizer::new("PRINT A NOPE");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::var('A')));
    assert_eq!(scanner.next_token(), Err("Invalid token at 'NOPE'".to_string()));
}