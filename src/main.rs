use tbac::compiler::compile;

fn main() {
    let tb_code = r#"LET C = 1
                    20 PRINT "HELLO ", C
                    LET C = C + 1
                    IF C > 10 THEN GOTO 60
                    GOTO 20
                    60 PRINT "END"
                   "#;

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", js_code) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
