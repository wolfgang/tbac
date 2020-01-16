use crate::code_generator::generate_code;

pub fn compile(code: &str) -> Result<String, String> {
    let runtime = "let c = 0; let r = true;function goto(n) { c = n } while (r) { switch (c) { %%CODE%% default: r = false; } }";

    match generate_code(code) {
        Ok(js_code) => { Ok(runtime.replace("%%CODE%%", js_code.as_str())) }
        Err(error) => { Err(error) }
    }
}
