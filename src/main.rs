use tbac::compiler::compile;

fn main() {
    let tb_code = r#"LET A = 10
                    IF A > 5 THEN PRINT "A is > 5"
                    IF A < 100 THEN PRINT "A is < 100"
                    IF A < 5 THEN PRINT "NOBODY WILL SE THIS"
                    PRINT "THE END""#;

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", js_code) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
