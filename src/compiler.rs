use crate::tokenizer::tokenize;
use crate::parser::parser::Parser;
use crate::code_generator::CodeGenerator;

pub fn compile(code: &str) -> Result<String, String> {
    let tokens = tokenize(code)?;
    let root = Parser::new(&tokens).parse()?;
    Ok(CodeGenerator {}.generate(&root))
}
