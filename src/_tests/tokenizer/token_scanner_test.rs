
use crate::tokenizer::Token;
use crate::tokenizer::token_scanner::TokenScanner;


#[test]
fn scan_uppercase_tokens() {
    let mut scanner = TokenScanner::new("PRINT IF LET THEN");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::iff()));
    assert_eq!(scanner.next_token(), Ok(Token::lett()));
    assert_eq!(scanner.next_token(), Ok(Token::then()));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_number_tokens() {
    let mut scanner = TokenScanner::new("PRINT 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_string_tokens() {
    let mut scanner = TokenScanner::new(r#"PRINT "abcdABCD!""#);
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::string("abcdABCD!")));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_relop_tokens() {
    let mut scanner = TokenScanner::new("PRINT < > =");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::relop('<')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('>')));
    assert_eq!(scanner.next_token(), Ok(Token::relop('=')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_termop_and_factop_tokens() {
    let mut scanner = TokenScanner::new("PRINT + - * /");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::termop('+')));
    assert_eq!(scanner.next_token(), Ok(Token::termop('-')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('*')));
    assert_eq!(scanner.next_token(), Ok(Token::factop('/')));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_vars() {
    let mut scanner = TokenScanner::new("PRINT A Z");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::var('A')));
    assert_eq!(scanner.next_token(), Ok(Token::var('Z')));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn scan_comma_and_friends() {
    let mut scanner = TokenScanner::new("PRINT , ( )");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::comma()));
    assert_eq!(scanner.next_token(), Ok(Token::openbracket()));
    assert_eq!(scanner.next_token(), Ok(Token::closebracket()));

    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}

#[test]
fn handles_invalid_token_in_the_middle_of_valid_ones() {
    let mut scanner = TokenScanner::new("PRINT x 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Err("Invalid token".to_string()));
    assert_eq!(scanner.next_token(), Err("Invalid token".to_string()));
}

#[test]
fn handles_newlines() {
    let mut scanner = TokenScanner::new("\n   PRINT \n\n 1234");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
}

#[test]
fn handles_trailing_whitespace() {
    let mut scanner = TokenScanner::new("PRINT 1234     \n ");
    assert_eq!(scanner.next_token(), Ok(Token::print()));
    assert_eq!(scanner.next_token(), Ok(Token::number(1234)));
    assert_eq!(scanner.next_token(), Ok(Token::end_of_stream()));
}