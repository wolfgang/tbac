use crate::code_generator::generate_code;

pub fn compile(code: &str) -> Result<String, String> {
    let runtime = r#"let c_ = 0; let r_ = true;
                     function goto(n) { c_ = n }
                     while (r_) { switch (c_) { %%CODE%% default: r_ = false; } }
                   "#;

    match generate_code(code) {
        Ok(js_code) => { Ok(runtime.replace("%%CODE%%", js_code.as_str())) }
        Err(error) => { Err(error) }
    }
}
