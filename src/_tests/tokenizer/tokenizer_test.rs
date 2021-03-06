use crate::tokenizer::Token;
use crate::tokenizer::tokenize;

#[test]
fn tokenizes_print_statement_without_parameters() {
    assert_eq!(tokenize("PRINT"),
               Ok(vec![Token::print()]))
}

#[test]
fn tokenizes_all_keywords() {
    assert_eq!(tokenize("PRINT IF THEN LET GOTO"),
               Ok(vec![
                   Token::print(),
                   Token::iff(),
                   Token::then(),
                   Token::lett(),
                   Token::goto()]))
}

#[test]
fn tokenizes_string_literals() {
    assert_eq!(tokenize(r#"PRINT "hello" PRINT "world""#),
               Ok(vec![
                   Token::print(),
                   Token::string("hello"),
                   Token::print(),
                   Token::string("world")]))
}

#[test]
fn tokenizes_numbers() {
    assert_eq!(tokenize("PRINT 12345"),
               Ok(vec![
                   Token::print(),
                   Token::number(12345)]))
}

#[test]
fn tokenize_gt() {
    assert_eq!(tokenize("12 > 10"),
               Ok(vec![
                   Token::number(12),
                   Token::relop('>'),
                   Token::number(10)]))
}

#[test]
fn tokenize_relops() {
    assert_eq!(tokenize("> < ="),
               Ok(vec![
                   Token::relop('>'),
                   Token::relop('<'),
                   Token::relop('=')]))
}

#[test]
fn tokenize_vars() {
    assert_eq!(tokenize("A PRINT B 1234"),
               Ok(vec![
                   Token::var('A'),
                   Token::print(),
                   Token::var('B'),
                   Token::number(1234)]))
}

#[test]
fn tokenize_commas() {
    assert_eq!(tokenize("A,B,C"),
               Ok(vec![
                   Token::var('A'),
                   Token::comma(),
                   Token::var('B'),
                   Token::comma(),
                   Token::var('C')]))
}

#[test]
fn tokenize_termops() {
    assert_eq!(tokenize("+ -"),
               Ok(vec![
                   Token::termop('+'),
                   Token::termop('-')]))
}

#[test]
fn tokenize_factops() {
    assert_eq!(tokenize("* /"),
               Ok(vec![
                   Token::factop('*'),
                   Token::factop('/')]))
}

#[test]
fn tokenize_brackets() {
    assert_eq!(tokenize("( )"),
               Ok(vec![
                   Token::openbracket(),
                   Token::closebracket()]))
}


#[test]
fn handles_surrounding_whitespace() {
    assert_eq!(tokenize("   \n A B <  \n  "),
               Ok(vec![
                   Token::var('A'),
                   Token::var('B'),
                   Token::relop('<'), ]))
}

#[test]
fn handles_blanks_inside_string_literals() {
    assert_eq!(tokenize(r#"PRINT " hello world   lol ""#),
               Ok(vec![
                   Token::print(),
                   Token::string(" hello world   lol ")]))
}

#[test]
fn handles_some_special_chars_inside_strings() {
    assert_eq!(tokenize(r#"PRINT "> < = : + -""#),
               Ok(vec![
                   Token::print(),
                   Token::string("> < = : + -")]))
}


#[test]
fn returns_error_if_string_is_unterminated() {
    assert_eq!(tokenize(r#"PRINT "hello" PRINT "hellooo"#),
               Err("Invalid token at '\"hellooo'".to_string()));
}

#[test]
fn returns_error_if_encountering_unknown_token() {
    assert_eq!(tokenize("PRINT 123 %"),
               Err("Invalid token at '%'".to_string()));
}