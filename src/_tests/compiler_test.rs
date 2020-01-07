fn compile(_code: &str) -> &str {
    return "to be implemented";
}

#[ignore]
#[test]
fn compiles_if_statement() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES""#;
    let expected_js_code = r#"if (2 > 1) { console.log("YES") }"#;

    let js_code = compile(tb_code);

    assert_eq!(js_code, expected_js_code);


}