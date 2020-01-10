use crate::code_generator::CodeGenerator;
use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello world"));

    assert_eq!(generate_code(&root),
               "console.log('hello world');\n")
}

#[test]
fn generate_if_statement() {
    let mut root = SequenceNode::new();
    root.add(
        IfNode::new(
            NumberNode::new(10),
            NumberNode::new(20),
            '<',
            PrintNode::new("hello")));

    assert_eq!(generate_code(&root),
               "if (10 < 20) {\n  console.log('hello');\n}\n");
}

#[test]
fn generate_let_statement() {
    let mut root = SequenceNode::new();
    root.add(LetNode::new('A', NumberNode::new(1234)));

    assert_eq!(generate_code(&root),
               "let A = 1234;\n");
}

#[test]
fn generate_multiple_statements() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello"));
    root.add(PrintNode::new("world"));

    assert_eq!(generate_code(&root),
               "console.log('hello');\nconsole.log('world');\n");
}

fn generate_code(root: &SequenceNode) -> String {
    CodeGenerator {}.generate(&root)
}