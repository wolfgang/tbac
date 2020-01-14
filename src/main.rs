use tbac::compiler::compile;

fn main() {
    let tb_code = r#"LET A = 10
                    IF A > 5 THEN PRINT "A is > 5"
                    IF A < 100 THEN IF A < 50 THEN PRINT "A is < 100 and < 50"
                    IF A < 5 THEN PRINT "NOBODY WILL SEE THIS"
                    PRINT "THE END"
                    PRINT "HELLO", 1234, "WORLD", 121223
                    PRINT "The value of A is:", A
                   "#;

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", js_code) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
