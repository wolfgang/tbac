use crate::code_generator::CodeGenerator;
use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::number_node::NumberNode;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::string_node::StringNode;
use crate::parser::expression_node::ExpressionNode;
use crate::parser::var_node::VarNode;

#[test]
fn generate_print_statement() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::with_string_param("hello world"));

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
            PrintNode::with_string_param("hello")));

    assert_eq!(generate_code(&root),
               "if (10 < 20) {\n  console.log('hello');\n}\n");
}

#[test]
fn generate_let_statement() {
    let mut root = SequenceNode::new();
    root.add(LetNode::new('A', NumberNode::new(1234)));

    assert_eq!(generate_code(&root),
               "A = 1234;\n");
}

#[test]
fn generate_let_statement_with_expression() {
    let mut root = SequenceNode::new();
    root.add(
        LetNode::new(
            'A',
            ExpressionNode::new(
                '-',
                NumberNode::new(1234),
                VarNode::new('B'))));

    assert_eq!(generate_code(&root),
               "A = 1234 - B;\n");
}


#[test]
fn generate_multiple_statements() {
    let mut root = SequenceNode::new();
    root.add(PrintNode::with_string_param("hello"));
    root.add(PrintNode::with_string_param("world"));

    assert_eq!(generate_code(&root),
               "console.log('hello');\nconsole.log('world');\n");
}

#[test]
fn generate_print_statement_with_multiple_params() {
    let mut root = SequenceNode::new();
    let mut print_node = PrintNode::new();
    print_node.add_param(StringNode::new("hello"));
    print_node.add_param(NumberNode::new(1234));

    root.add(print_node);

    assert_eq!(generate_code(&root),
               "console.log('hello');console.log(1234);\n")
}


fn generate_code(root: &SequenceNode) -> String {
    CodeGenerator {}.generate(&root)
}