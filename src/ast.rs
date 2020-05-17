use crate::scanner::Token;

pub enum AST {
    Program(Token, Vec<Subroutine>, Vec<Statement>),
}

pub enum Subroutine {
    Function(Token, Vec<(Token, TypeDescription)>, Vec<Statement>),
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

pub enum SimpleType {
    Boolean,
    String,
    Real,
    Integer,
}

pub enum NodeType {
    Simple(SimpleType),
    ArrayOf(SimpleType),
}

pub enum TypedAST {
    Program(Token, Vec<TypedSubroutine>, Vec<TypedStatement>),
}

pub enum TypedTypeDescription {
    Simple(Token),
    Array(Token, TypedExpression),
}

pub enum TypedSubroutine {
    Function(Token, Vec<(Token, TypeDescription)>, Vec<TypedStatement>),
    Procedure(Token, Vec<(Token, TypeDescription)>, Vec<TypedStatement>),
}


pub enum TypedVariableCore {
    Simple(Token),
    Indexed(Token, SimplyTypedExpression),
}

pub enum SimplyTypedVariable {
    Int(TypedVariableCore),
    Boolean(TypedVariableCore),
    String(TypedVariableCore),
    Real(TypedVariableCore),
}
    

pub enum  TypedVariable {
    Simple(SimplyTypedVariable, u64),
    ArrayOf(SimplyTypedVariable, u64),
}
    
pub enum TypedStatement {
    Assign(TypedVariable, TypedExpression),
    Declaration(Token, TypedTypeDescription),
    Block(Vec<TypedStatement>),
    If(
        SimplyTypedExpression,
        Box<TypedStatement>,
        Option<Box<TypedStatement>>,
    ),
    While(SimplyTypedExpression, Box<TypedStatement>),
    Assert(Token, SimplyTypedExpression),
    Call(Token, Vec<TypedExpression>),
    Return(Token, Option<TypedExpression>),
}

pub enum TypedExpression {
    Simple(SimplyTypedExpression, u64),
    ArrayOf(SimplyTypedExpression, u64),
}

pub enum SimplyTypedExpression {
    Int(TypedExpressionCore, u64),
    Boolean(TypedExpressionCore, u64),
    String(TypedExpressionCore, u64),
    Real(TypedExpressionCore, u64),
}

pub enum TypedExpressionCore {
    Literal(Token),
    Variable(Box<TypedVariable>),
    Binary(Box<SimplyTypedExpression>, Box<SimplyTypedExpression>, Token),
    Unary(Box<SimplyTypedExpression>, Token),
    Call(Token, Vec<TypedExpression>),
}


