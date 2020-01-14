use crate::compiler::compile;

#[test]
fn compiles_if_statement() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES""#;
    let expected_js_code = "if (2 > 1) {\n  console.log('YES');\n}\n";

    let result = compile(tb_code);

    assert_eq!(result, Ok(expected_js_code.to_string()));
}

#[test]
fn compiles_let_statement() {
    let tb_code = "LET A = 1234";
    let expected_js_code = "A = 1234;\n";

    let result = compile(tb_code);

    assert_eq!(result, Ok(expected_js_code.to_string()));
}

#[test]
fn compiles_if_statement_with_variable() {
    let tb_code = r#"LET A = 10
                     IF A > 20 THEN PRINT "YES""#;
    let expected_js_code = "A = 10;\nif (A > 20) {\n  console.log('YES');\n}\n";

    let result = compile(tb_code);

    assert_eq!(result, Ok(expected_js_code.to_string()));
}

#[test]
fn compiles_print_statement_with_multiple_args() {
    let tb_code = r#"PRINT 1234,"hello", A"#;
    let expected_js_code = "console.log(1234);console.log('hello');console.log(A);\n";

    let result = compile(tb_code);

    assert_eq!(result, Ok(expected_js_code.to_string()));
}


#[test]
fn returns_error_if_tokenizing_goes_wrong() {
    verify_error("what",
                 "Unrecognized character 'w'");
}

#[test]
fn returns_error_if_parsing_goes_wrong() {
    verify_error("THEN IF",
                 "Expected command token but got THEN");
}

fn verify_error(code: &str, expected_error: &str) {
    assert_eq!(compile(code), Err(expected_error.to_string()));
}