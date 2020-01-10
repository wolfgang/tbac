use crate::code_generator::CodeGenerator;
use crate::parser::ifnode::IfNode;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello world"));

    assert_eq!(generate_code(&root),
               "console.log('hello world');")
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
               "if (10 < 20) { console.log('hello'); }\n");
}

#[test]
fn generate_multiple_statements() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello"));
    root.add(PrintNode::new("world"));

    assert_eq!(generate_code(&root),
               "console.log('hello');console.log('world');");
}

fn generate_code(root: &SequenceNode) -> String {
    CodeGenerator {}.generate(&root)
}