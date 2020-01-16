use tbac::compiler::compile;

fn main() {
    let tb_code = r#"LET C = 1
                    20 PRINT "HELLO ", C
                    LET C = C + 1
                    IF C > 10 THEN GOTO 60
                    GOTO 20
                    60 PRINT "END"
                   "#;


    let runtime = "let c = 0; let r = true;function goto(n) { c = n } while (r) { switch (c) { %%CODE%% default: r = false; } }";

    match compile(tb_code) {
        Ok(js_code) => { println!("{}", runtime.replace("%%CODE%%", js_code.as_str())) }
        Err(error) => { println!("Compiler error: {}", error) }
    }
}
