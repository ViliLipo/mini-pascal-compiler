use crate::token::Token;
use crate::address::Address;
use crate::opkind::OpKind;

#[derive(PartialEq, Clone, Debug)]
pub enum SimpleType {
    Boolean,
    String,
    Real,
    Integer,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NodeType {
    Simple(SimpleType),
    ArrayOf(SimpleType),
}

pub enum TypedAST {
    Program(Token, Vec<TypedSubroutine>, Vec<TypedStatement>),
}

pub enum TypedTypeDescription {
    Simple(NodeType),
    Array(NodeType, TypedExpression),
}

pub enum TypedSubroutine {
    Function(
        Address,
        Vec<(TypedVariable, TypedTypeDescription)>,
        Vec<TypedStatement>,
        TypedTypeDescription,
    ),
    Procedure(
        Address,
        Vec<(TypedVariable, TypedTypeDescription)>,
        Vec<TypedStatement>,
    ),
}

pub struct TypedVariable {
    pub token: Token,
    pub address: Address,
    pub node_type: NodeType,
    pub substructure: TypedVariableStructure,
}

pub enum TypedVariableStructure {
    Simple,
    Indexed(TypedExpression),
}

pub enum TypedStatement {
    Assert(Token, TypedExpression),
    Assign(TypedVariable, TypedExpression),
    Block(Vec<TypedStatement>),
    Call(Address, Vec<TypedExpression>),
    Declaration(TypedVariable, TypedTypeDescription),
    If(
        TypedExpression,
        Box<TypedStatement>,
        Option<Box<TypedStatement>>,
    ),
    Read(Vec<Box<TypedVariable>>),
    Return(Token, Option<TypedExpression>),
    While(TypedExpression, Box<TypedStatement>),
    Write(Vec<TypedExpression>),
}

pub struct TypedExpression {
    pub token: Token,
    pub address: Address,
    pub node_type: NodeType,
    pub substructure: TypedExpressionStructure,
}

pub enum TypedExpressionStructure {
    Binary(OpKind, Box<TypedExpression>, Box<TypedExpression>),
    Call(Address, Vec<TypedExpression>),
    Literal,
    Size(Address),
    Unary(Box<TypedExpression>),
    Variable(Box<TypedVariable>),
}

