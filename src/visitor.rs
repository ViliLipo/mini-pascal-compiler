use crate::ast::*;

pub trait Visitor {
    fn visit(&mut self, node: &mut dyn Node);
    fn visit_program(&mut self, node: &mut Program);
    fn visit_block(&mut self, node: &mut Block);
    fn visit_declaration(&mut self, node: &mut Declaration);
    fn visit_assignment(&mut self, node: &mut Assignment);
    fn visit_expression(&mut self, node: &mut Expression);
    fn visit_identifier(&mut self, node: &mut Identifier);
    fn visit_variable(&mut self, node: &mut Variable);
    fn visit_literal(&mut self, node: &mut Literal);
    fn visit_call(&mut self, node: &mut Call);
    fn visit_if(&mut self, node: &mut IfNode);
    fn visit_while(&mut self, node: &mut WhileNode);
    fn visit_argument(&mut self, node: &mut ArgumentNode);
    fn visit_assert(&mut self, node: &mut AssertNode);
}
