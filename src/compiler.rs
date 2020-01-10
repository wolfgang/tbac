use crate::tokenizer::Tokenizer;
use crate::parser::parser::Parser;
use crate::code_generator::CodeGenerator;

pub fn compile(code: &str) -> String {
    let tokens = Tokenizer::new(code).tokenize().unwrap();
    let root = Parser::new(&tokens).parse().unwrap();
    CodeGenerator {}.generate(&root)
}
