use crate::compiler::compile;

#[test]
fn compiles_if_statement() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES""#;
    let expected_js_code = "if (2 > 1) { console.log('YES'); }\n";

    let js_code = compile(tb_code);

    assert_eq!(js_code, expected_js_code);


}