use tbac::compiler::compile;

fn main() {
    let tb_code = r#"IF 2 > 1 THEN PRINT "YES"
                    PRINT "THE END""#;

    let js_code = compile(tb_code);

    println!("{}", js_code);
}
