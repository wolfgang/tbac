trait Node {
    fn eval(&self, code_generator: &mut dyn CodeGenerator);
}

struct PrintNode {
    string_param: String
}

impl PrintNode {
    pub fn new(param: &str) -> Self {
        Self { string_param: param.to_string() }
    }
}

impl Node for PrintNode {
    fn eval(&self, code_generator: &mut dyn CodeGenerator) {
        code_generator.gen_print(&self);
    }
}

trait CodeGenerator {
    fn gen_print(&mut self, node: &PrintNode);
}


struct TestCodeGenerator {
    code: Vec<String>
}

impl TestCodeGenerator {
    pub fn new() -> Self {
        Self { code: Vec::with_capacity(32) }
    }
}

impl CodeGenerator for TestCodeGenerator {
    fn gen_print(&mut self, node: &PrintNode) {
        self.code.push(format!("print {}", node.string_param));
    }
}


#[test]
fn print_node_eval() {
    let node = PrintNode::new("hello");
    let mut code_generator = TestCodeGenerator::new();

    node.eval(&mut code_generator);

    assert_eq!(code_generator.code, vec!["print hello"]);
}