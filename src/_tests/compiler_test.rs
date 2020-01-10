use crate::compiler::compile;

#[test]
fn compiles_if_statement() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES""#;
    let expected_js_code = "if (2 > 1) { console.log('YES'); }\n";

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