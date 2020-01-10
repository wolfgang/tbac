use tbac::compiler::compile;

fn main() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES"
                    PRINT "THE END""#;

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", js_code) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
