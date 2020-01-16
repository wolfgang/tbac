use crate::compiler::compile;
use crate::tokenizer::tokenize;
use crate::parser::parser::Parser;
use crate::code_generator::CodeGenerator;

#[test]
fn compiles_if_statement_with_expressions() {
    verify_generated_code(
        r#"LET A = 10
           LET B = 20
           IF A + B > A - B THEN PRINT "YES""#,
        &vec![
            "A = 10;",
            "B = 20;",
            "if (A + B > A - B) {",
            "  console.log('YES');",
            "}",
            ""
        ].join("\n"));
}

#[test]
fn compiles_print_statement_with_multiple_args() {
    verify_generated_code(r#"PRINT 1234,"hello", A"#,
                          "console.log(1234);console.log('hello');console.log(A);\n");
}

#[test]
fn compiles_print_statement_with_expressions() {
    verify_generated_code(r#"PRINT 1234 + A, B+23"#,
                          "console.log(1234 + A);console.log(B + 23);\n");
}

#[test]
fn compiles_let_with_complex_expressions() {
    verify_generated_code("LET A = 1 + 2 - 3 + B + 4",
                          "A = 1 + 2 - 3 + B + 4;\n");
}

#[test]
fn compiles_brackets() {
    verify_generated_code("LET A = 1 * (2 + 3 * (4 + 5))",
                          "A = 1 * (2 + 3 * (4 + 5));\n");
}


#[test]
fn returns_error_if_tokenizing_goes_wrong() {
    verify_error("what",
                 "Invalid token at 'what'");
}

#[test]
fn returns_error_if_parsing_goes_wrong() {
    verify_error("THEN IF",
                 "Expected command token but got Then");
}

fn verify_generated_code(tb_code: &str, expected_js_code: &str) {
    let result = generate_code(tb_code);
    assert_eq!(result, Ok(expected_js_code.to_string()));
}

fn verify_error(code: &str, expected_error: &str) {
    assert_eq!(compile(code), Err(expected_error.to_string()));
}

pub fn generate_code(code: &str) -> Result<String, String> {
    let tokens = tokenize(code)?;
    let root = Parser::new(&tokens).parse()?;
    Ok(CodeGenerator {}.generate(&root))
}
