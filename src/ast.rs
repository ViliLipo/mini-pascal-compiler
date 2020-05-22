use std::fmt;
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
    Simple(Token),
    Array(Token, TypedExpression),
}

pub enum TypedSubroutine {
    Function(Address, Vec<(TypedVariable, TypedTypeDescription)>, Vec<TypedStatement>),
    Procedure(Address, Vec<(TypedVariable, TypedTypeDescription)>, Vec<TypedStatement>),
}

pub struct TypedVariable {
    pub token: Token,
    pub address: Address,
    pub node_type: NodeType,
    pub substructure: TypedVariableStructure,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    data: AddressData,
}

impl Address {
    pub fn new_simple(address: u64) -> Address {
        Address{
            data: AddressData::Simple(address),
        }
    }

    pub fn new_indexed(address:Address, index: Address) -> Address {
        Address{
            data: AddressData::Indexed(address.as_u64(), index.as_u64()),
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self.data {
            AddressData::Simple(address) => address,
            AddressData::Indexed(address, _index) => address,
        }
    }

    pub fn register_format(&self) -> String {
        match self.data {
            AddressData::Simple(address) => {
                format!("r{}", address)
            },
            AddressData::Indexed(address, index) => {
                format!("r{}[r{}]", address, index)
            }
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.register_format())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum AddressData {
    Simple(u64),
    Indexed(u64, u64)
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
    Binary(Box<TypedExpression>, Box<TypedExpression>),
    Call(Address, Vec<TypedExpression>),
    Literal,
    Size(Address),
    Unary(Box<TypedExpression>),
    Variable(Box<TypedVariable>),
}
