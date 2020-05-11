use crate::ast::*;
use crate::visitor::Visitor;

pub struct PrintVisitor {}

impl PrintVisitor {}
impl Visitor for PrintVisitor {
    fn visit(&mut self, node: &mut dyn Node) {
        println!("Generic Node");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
    }
    fn visit_program(&mut self, node: &mut Program) {
        print!("Program(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_block(&mut self, node: &mut Block) {
        print!("Block(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_declaration(&mut self, node: &mut Declaration) {
        print!("Declaration(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_assignment(&mut self, node: &mut Assignment) {
        print!("Assignment(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_expression(&mut self, node: &mut Expression) {
        let text = format!("Expression: {} (", node.get_token().lexeme);
        print!("{}", text);
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_variable(&mut self, node: &mut Variable) {
        let text = format!("Variable: {} (", node.get_token().lexeme);
        print!("{}", text);
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
    fn visit_literal(&mut self, node: &mut Literal) {
        print!("Literal(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }

    fn visit_call(&mut self, node: &mut Call) {
        print!("Call(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }

    fn visit_if(&mut self, node: &mut IfNode) {
        print!("If(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }

    fn visit_while(&mut self, node: &mut WhileNode) {
        print!("While(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        print!("Identifier(");
        print!("{}", node.get_token().lexeme);
        print!(")");
    }

    fn visit_argument(&mut self, node: &mut ArgumentNode) {
        print!("Arguments(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }

    fn visit_assert(&mut self, node: &mut AssertNode) {
        print!("Assert(");
        for child in node.get_children() {
            println!("");
            child.accept(self);
        }
        print!(")");
    }
}


