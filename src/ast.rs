use crate::token::Token;

pub enum AST {
    Program(Token, Vec<Subroutine>, Vec<Statement>),
}

pub enum Subroutine {
    Function(
        Token,
        Vec<(Token, TypeDescription)>,
        Vec<Statement>,
        TypeDescription,
    ),
    Procedure(Token, Vec<(Token, TypeDescription)>, Vec<Statement>),
}

pub enum Statement {
    Assign(Box<Variable>, Expression),
    Declaration(Token, TypeDescription),
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Assert(Token, Expression),
    Call(Token, Vec<Expression>),
    Return(Token, Option<Expression>),
}

pub enum Expression {
    Literal(Token),
    Variable(Box<Variable>),
    Binary(Box<Expression>, Box<Expression>, Token),
    Unary(Box<Expression>, Token),
    Call(Token, Vec<Expression>),
}

pub enum TypeDescription {
    Simple(Token),
    Array(Token, Expression),
}

pub enum Variable {
    Simple(Token),
    Indexed(Token, Expression),
}

