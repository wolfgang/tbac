use crate::code_generator::CodeGenerator;
use crate::parser::ifnode::IfNode;
use crate::parser::node::Node;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello world"));

    let mut code_generator = CodeGenerator::new();

    let code = root.eval(&mut code_generator);

    assert_eq!(code, r#"console.log("hello world")"#)
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

    let mut code_generator = CodeGenerator::new();
    let code = root.eval(&mut code_generator);
    assert_eq!(code, r#"if (10 < 20) { console.log("hello") }"#)
}