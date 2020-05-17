use crate::ast::*;
use crate::scanner::Token;

pub trait Visitor {
    fn visit_ast(&mut self, node: &AST);
    fn visit_subroutine(&mut self, node: &Subroutine);
    fn visit_block(&mut self, node: &Vec<Statement>);
    fn visit_statement(&mut self, node: &Statement);
    fn visit_assign(&mut self, variable: &Variable, value:&Expression);
    fn visit_declaration(&mut self, identifier: &Token, type_description: &TypeDescription);
    fn visit_if(
        &mut self,
        condition: &Expression,
        body: &Statement,
        else_body: Option<&Statement>,
    );
    fn visit_while(&mut self, condition: &Expression, body: &Statement);
    fn visit_assert(&mut self, token: &Token, condition: &Expression);
    fn visit_call(&mut self, token: &Token, arguments: &Vec<Expression>);
    fn visit_return(&mut self, token: &Token, value: &Option<Expression>);
    fn visit_expression(&mut self, node: &Expression);
    fn visit_literal(&mut self, token: &Token);
    fn visit_variable(&mut self, var: &Variable);
    fn visit_binary_expression(&mut self, lhs: &Expression, rhs: &Expression, op: &Token);
}
