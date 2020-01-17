use crate::parser::expression_node::ExpressionNode;
use crate::parser::goto_node::GotoNode;
use crate::parser::ifnode::IfNode;
use crate::parser::let_node::LetNode;
use crate::parser::node::{Node, NodeBox};
use crate::parser::node_evaluator::NodeEvaluator;
use crate::parser::number_node::NumberNode;
use crate::parser::parser::Parser;
use crate::parser::print_node::PrintNode;
use crate::parser::sequence_node::SequenceNode;
use crate::parser::string_node::StringNode;
use crate::parser::var_node::VarNode;
use crate::tokenizer::tokenize;

pub fn generate_code(code: &str) -> Result<String, String> {
    let tokens = tokenize(code)?;
    let root = Parser::new(&tokens).parse()?;
    Ok(CodeGenerator {}.generate(&root))
//    Ok(CodeGenerator {}.generate2(&root))
}


fn try_as_node<T>(node: &NodeBox) -> Option<&T> where T: Node {
    node.as_any().downcast_ref::<T>()
}


pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn generate(&self, root: &SequenceNode) -> String {
        root.eval(self)
    }

    pub fn generate2(&self, root: &SequenceNode) -> String {
        root.children.iter().fold(
            String::with_capacity(512),
            |acc, child| format!("{}case {}: {}", acc, child.line(), self.generate3(child)))
    }

    pub fn generate3(&self, node: &NodeBox) -> String {
        if let Some(print_node) = try_as_node::<PrintNode>(node) {
            let mut result = String::with_capacity(256);
            for param in &print_node.params {
                result.push_str(&format!("console.log({});", self.generate3(param)));
            }
            result.push('\n');
            return result;
        }

        if let Some(number_node) = try_as_node::<NumberNode>(node) {
            return number_node.value.to_string();
        }

        if let Some(if_node) = try_as_node::<IfNode>(node) {
            return format!("if ({} {} {}) {{\n  {}}}\n",
                           self.generate3(&if_node.left),
                           if_node.relop,
                           self.generate3(&if_node.right),
                           self.generate3(&if_node.then));
        }

        if let Some(let_node) = try_as_node::<LetNode>(node) {
            return format!("{} = {};\n", let_node.var, self.generate3(&let_node.value));
        }

        if let Some(goto_node) = try_as_node::<GotoNode>(node) {
            return format!("goto({}); break;\n", self.generate3(&goto_node.destination));
        }

        if let Some(var_node) = try_as_node::<VarNode>(node) {
            return var_node.var_name.to_string();
        }

        if let Some(string_node) = try_as_node::<StringNode>(node) {
            return format!("'{}'", string_node.value.clone());
        }

        if let Some(expression_node) = try_as_node::<ExpressionNode>(node) {
            let brackets = if expression_node.in_brackets { ("(", ")") } else { ("", "") };
            return format!("{}{} {} {}{}",
                           brackets.0,
                           self.generate3(&expression_node.left),
                           &expression_node.op,
                           self.generate3(&expression_node.right),
                           brackets.1);
        }


        "".to_string()
    }
}

impl NodeEvaluator for CodeGenerator {
    fn eval_sequence(&self, node: &SequenceNode) -> String {
        node.children.iter().fold(
            String::with_capacity(512),
            |acc, child| format!("{}case {}: {}", acc, child.line(), child.eval(self)))
    }

    fn eval_print(&self, node: &PrintNode) -> String {
        let mut result = String::with_capacity(256);
        for param in &node.params {
            result.push_str(&format!("console.log({});", param.eval(self)));
        }
        result.push('\n');
        result
    }

    fn eval_number(&self, node: &NumberNode) -> String {
        node.value.to_string()
    }

    fn eval_if(&self, node: &IfNode) -> String {
        format!("if ({} {} {}) {{\n  {}}}\n",
                node.left.eval(self),
                node.relop,
                node.right.eval(self),
                node.then.eval(self))
    }

    fn eval_let(&self, node: &LetNode) -> String {
        format!("{} = {};\n", node.var, node.value.eval(self))
    }

    fn eval_goto(&self, node: &GotoNode) -> String {
        format!("goto({}); break;\n", node.destination.eval(self))
    }

    fn eval_var(&self, node: &VarNode) -> String {
        node.var_name.to_string()
    }

    fn eval_string(&self, node: &StringNode) -> String {
        format!("'{}'", node.value.clone())
    }

    fn eval_expression(&self, node: &ExpressionNode) -> String {
        let brackets = if node.in_brackets { ("(", ")") } else { ("", "") };
        format!("{}{} {} {}{}",
                brackets.0,
                node.left.eval(self),
                node.op,
                node.right.eval(self),
                brackets.1)
    }
}
