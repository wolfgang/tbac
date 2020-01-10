use tbac::code_generator::CodeGenerator;
use tbac::parser::parser::Parser;
use tbac::tokenizer::Tokenizer;

fn main() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES"
                    PRINT "THE END""#;

    let tokens = Tokenizer::new(tb_code).tokenize().unwrap();
    let root = Parser::new(&tokens).parse().unwrap();
    let js_code = CodeGenerator {}.generate(&root);

    println!("{}", js_code);
}
