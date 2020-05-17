use crate::ast::*;
use crate::scanner::Token;
use crate::visitor::Visitor;

pub struct PrintVisitor {}

impl PrintVisitor{
    pub fn new() -> Box<dyn Visitor> {
        Box::from(PrintVisitor{})
    }
}

impl Visitor for PrintVisitor {
    fn visit_ast(&mut self, node: &AST) {
        match node {
            AST::Program(token, subroutines, main_block) => {
                println!("Program {} ->", token.lexeme);
                for subroutine in subroutines {
                    self.visit_subroutine(subroutine);
                }
                self.visit_block(main_block);
            }
        }
    }

    fn visit_subroutine(&mut self, node: &Subroutine) {} // TODO SUBROUTINES

    fn visit_block(&mut self, node: &Vec<Statement>) {
        println!("Block[");
        for statement in node{
            self.visit_statement(&statement);
        }
        print!("]");
    }
    fn visit_statement(&mut self, node: &Statement) {
        match node {
            Statement::Assign(var, ex) => self.visit_assign(var, ex),
            Statement::Declaration(token, type_description) => {
                self.visit_declaration(token, type_description)
            }
            Statement::While(condition, body) => self.visit_while(condition, body),
            Statement::If(condition, body, maybe_else_body) => {
                if let Some(else_body) = maybe_else_body {
                    self.visit_if(condition, body, Some(else_body))
                } else {
                    self.visit_if(condition, body, None)
                }
            },
            Statement::Block(block) => self.visit_block(block),
            Statement::Assert(token, expression) => self.visit_assert(token, expression),
            Statement::Call(id, params) => self.visit_call(id, params),
            Statement::Return(token, value) => self.visit_return(token, value),
        }
        println!();
    }
    fn visit_assign(&mut self, variable: &Variable, value: &Expression) {
        print!("Assign(");
        self.visit_variable(variable);
        print!(":= ");
        self.visit_expression(value);
        print!(")");
    }

    fn visit_declaration(&mut self, token: &Token, type_description: &TypeDescription) {
        print!("Declaration( {} : ", token.lexeme.clone());
        match type_description {
            TypeDescription::Simple(t) => print!("{}", t.lexeme.clone()),
            TypeDescription::Array(t, e) => {
                print!("{}[", t.lexeme.clone());
                self.visit_expression(&e);
                print!("]");
            }
        }
        print!(")");
    }

    fn visit_if(
        &mut self,
        condition: &Expression,
        body: &Statement,
        else_body: Option<&Statement>,
    ) {
        print!("If (");
        self.visit_expression(condition);
        self.visit_statement(body);
        if let Some(eb) = else_body {
            print!("Else(");
            self.visit_statement(eb);
            print!(")");
        }
    }

    fn visit_while(&mut self, condition: &Expression, body: &Statement) {
        print!("While (");
        self.visit_expression(condition);
        print!(" ) do \n");
        self.visit_statement(body);
        print!("end while");

    }
    fn visit_assert(&mut self, _token: &Token, condition: &Expression) {
        print!("assert(");
        self.visit_expression(condition);
        print!(")");
    }
    fn visit_call(&mut self, token: &Token, arguments: &Vec<Expression>) {
        print!("call {} with parameters (", token.lexeme);
        for arg in arguments {
            self.visit_expression(arg);
        }
        print!(")");
    }
    fn visit_return(&mut self, _token: &Token, value: &Option<Expression>) {
        print!("Return(");
        if let Some(val) = value {
            self.visit_expression(val);
        }
        print!(")");
    }

    fn visit_expression(&mut self, node: &Expression) {
        match node {
            Expression::Literal(t) => self.visit_literal(t),
            Expression::Variable(v) => self.visit_variable(v),
            Expression::Binary(lhs, rhs, op) => self.visit_binary_expression(lhs, rhs, op),
            Expression::Unary(_op, _rhs) => (), // TODO: UNARY
            Expression::Call(id, parameters) => self.visit_call(id, parameters),
        }
    }
    fn visit_literal(&mut self, token: &Token) {
        print!("{} ", token.lexeme);
    }
    fn visit_variable(&mut self, var: &Variable) {
        match var {
            Variable::Simple(t) => print!(" {} ", t.lexeme),
            Variable::Indexed(t, e) => {
                print!("{}[", t.lexeme);
                self.visit_expression(&e);
                print!("]");
            }
        }
    }
    fn visit_binary_expression(&mut self, lhs: &Expression, rhs: &Expression, op: &Token) {
        print!("(");
        self.visit_expression(&lhs);
        print!(" {} ", op.lexeme);
        self.visit_expression(&rhs);
        print!(")");
    }
}
