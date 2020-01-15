use tbac::compiler::compile;

fn main() {
    let tb_code = r#"LET A = 10
                    LET B = A + 17
                    IF A > 5 THEN PRINT "A is > 5"
                    IF A < 100 THEN IF A < 50 THEN PRINT "A is < 100 and < 50"
                    IF A < 5 THEN PRINT "NOBODY WILL SEE THIS"
                    PRINT "THE END"
                    PRINT "HELLO", 1234, "WORLD", 121223
                    PRINT "The value of A is:", A
                    PRINT "The value of B is:", B
                    LET C = A + B
                    PRINT "The value of A + B is: ", C
                   "#;

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", js_code) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
