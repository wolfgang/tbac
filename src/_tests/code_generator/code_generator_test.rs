use crate::parser::sequence_node::SequenceNode;
use crate::parser::print_node::PrintNode;
use crate::parser::node::Node;
use crate::code_generator::CodeGenerator;

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::new("hello world"));

    let mut code_generator = CodeGenerator::new();

    let code = root.eval(&mut code_generator);

    assert_eq!(code, r#"console.log("hello world")"#)
}