use crate::tokenizer::Tokenizer;
use crate::parser::parser::Parser;
use crate::code_generator::CodeGenerator;

fn compile(code: &str) -> String {
    let tokens = Tokenizer::new(code).tokenize().unwrap();
    let root = Parser::new(&tokens).parse().unwrap();
    CodeGenerator {}.generate(&root)

}

#[test]
fn compiles_if_statement() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES""#;
    let expected_js_code = "if (2 > 1) { console.log('YES'); }\n";

    let js_code = compile(tb_code);

    assert_eq!(js_code, expected_js_code);


}