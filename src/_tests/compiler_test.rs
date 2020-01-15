use crate::compiler::compile;

#[test]
fn compiles_if_statement_with_expressions() {
    let tb_code = r#"LET A = 10
                     LET B = 20
                     IF A + B > A - B THEN PRINT "YES""#;
    let expected_js_code = vec![
        "A = 10;",
        "B = 20;",
        "if (A + B > A - B) {",
        "  console.log('YES');",
        "}",
        ""
    ].join("\n");

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
fn compiles_print_statement_with_expressions() {
    let tb_code = r#"PRINT 1234 + A, B+23"#;
    let expected_js_code = "console.log(1234 + A);console.log(B + 23);\n";

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
                 "Expected command token but got Then");
}

fn verify_error(code: &str, expected_error: &str) {
    assert_eq!(compile(code), Err(expected_error.to_string()));
}