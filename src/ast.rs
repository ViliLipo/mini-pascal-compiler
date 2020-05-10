use crate::scanner::Token;
use crate::scanner::TokenKind;
use crate::visitor::Visitor;
use std::any::Any;
use std::fmt;

#[derive(Debug)]
pub enum NodeType {
    Simple(String),
    ArrayOf(String),
    Unit,
}

impl NodeType {
    fn to_string(&self) -> String {
        match self {
            NodeType::Simple(s) => format!("Simple: {}", s),
            NodeType::ArrayOf(s) => format!("Array of: {}", s),
            NodeType::Unit => format!("Unit"),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            NodeType::Simple(s) => match other {
                NodeType::Simple(s2) => s == s2,
                _ => false,
            },
            NodeType::ArrayOf(s) => match other {
                NodeType::ArrayOf(s2) => s == s2,
                _ => false,
            },
            NodeType::Unit => match other {
                NodeType::Unit => true,
                _ => false,
            },
        }
    }
}

impl Clone for NodeType {
    fn clone(&self) -> NodeType {
        match self {
            NodeType::Unit => NodeType::Unit,
            NodeType::ArrayOf(s) => NodeType::ArrayOf(s.clone()),
            NodeType::Simple(s) => NodeType::Simple(s.clone()),
        }
    }
}

pub trait Node {
    fn get_token(&self) -> Token;
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>>;
    fn add_child(&mut self, child: Box<dyn Node>);
    fn accept(&mut self, visitor: &mut dyn Visitor);
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> NodeType;
    fn set_type(&mut self, node_type: NodeType);
    fn get_result_addr(&self) -> String;
    fn set_result_addr(&mut self, address: String);
}

pub struct Program {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Program {
    pub fn get_id_child(&self) -> Option<&Variable> {
        self.get_child(0)
    }
    fn get_child(&self, no: usize) -> Option<&Variable> {
        match self.children.get(no) {
            Some(boxed_child) => match boxed_child.as_any().downcast_ref::<Variable>() {
                Some(var) => Some(var),
                None => None,
            },
            None => None,
        }
    }
}

impl Node for Program {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_program(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Simple(String::from("integer"))
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
}

pub struct Declaration {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Declaration {
    pub fn get_id_child(&self) -> Option<&Identifier> {
        self.get_child(0)
    }
    fn get_child(&self, no: usize) -> Option<&Identifier> {
        match self.children.get(no) {
            Some(boxed_child) => match boxed_child.as_any().downcast_ref::<Identifier>() {
                Some(var) => Some(var),
                None => None,
            },
            None => None,
        }
    }
    pub fn get_type_child(&self) -> Option<&Identifier> {
        self.get_child(1)
    }

    pub fn get_array_type_len_child(&mut self) -> Option<&mut Box<dyn Node>> {
        match self.children.get_mut(2) {
            Some(child) => Some(child),
            None => None,
        }
    }
}

impl Node for Declaration {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_declaration(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
}

pub struct Block {
    token: Token,
    children: Vec<Box<dyn Node>>,
    scope_no: i32,
}

impl Block {
    pub fn get_scope_no(&self) -> i32 {
        self.scope_no
    }

    pub fn set_scope_no(&mut self, scope_no: i32) {
        self.scope_no = scope_no
    }
}

impl Node for Block {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_block(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
}

pub struct Assignment {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Assignment {
    pub fn get_rhs_child(&self) -> Option<&Box<dyn Node>> {
        match self.children.get(1) {
            Some(child) => Some(child),
            None => None,
        }
    }
    pub fn get_lhs_child(&self) -> Option<&Variable> {
        match self.children.get(0) {
            Some(boxed_child) => match boxed_child.as_any().downcast_ref::<Variable>() {
                Some(var) => Some(var),
                None => None,
            },
            None => None,
        }
    }
}
impl Node for Assignment {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_assignment(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
}

pub struct Expression {
    token: Token,
    children: Vec<Box<dyn Node>>,
    type_id: NodeType,
    result_addr: String,
}

impl Expression {
    pub fn get_lhs_child(&self) -> Option<&Box<dyn Node>> {
        self.children.get(0)
    }
    pub fn get_rhs_child(&self) -> Option<&Box<dyn Node>> {
        self.children.get(1)
    }
}

impl Node for Expression {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, type_id: NodeType) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> NodeType {
        self.type_id.clone()
    }

    fn set_result_addr(&mut self, addr: String) {
        self.result_addr = addr;
    }
    fn get_result_addr(&self) -> String {
        self.result_addr.clone()
    }
}

pub struct Variable {
    token: Token,
    children: Vec<Box<dyn Node>>,
    type_id: NodeType,
    result_addr: String,
}

impl Variable {
    pub fn has_index(&self) -> bool {
        self.children.len() > 0
    }
    pub fn get_index_child(&self) -> Option<&Box<dyn Node>> {
        self.children.get(0)
    }
}

impl Node for Variable {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_variable(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, type_id: NodeType) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> NodeType {
        self.type_id.clone()
    }
    fn set_result_addr(&mut self, addr: String) {
        self.result_addr = addr;
    }
    fn get_result_addr(&self) -> String {
        self.result_addr.clone()
    }
}

pub struct Identifier {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for Identifier {
    fn get_token(&self) -> Token {
        self.token.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn add_child(&mut self, _child: Box<dyn Node>) {
        ()
    }

    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        self.children.as_mut()
    }

    fn get_result_addr(&self) -> String {
        String::from("No address")
    }

    fn set_result_addr(&mut self, _addr: String) {
        ()
    }

    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }

    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

pub struct Literal {
    token: Token,
    children: Vec<Box<dyn Node>>,
    type_id: NodeType,
    result_addr: String,
}

impl Node for Literal {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_literal(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, type_id: NodeType) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> NodeType {
        self.type_id.clone()
    }
    fn set_result_addr(&mut self, addr: String) {
        self.result_addr = addr;
    }
    fn get_result_addr(&self) -> String {
        self.result_addr.clone()
    }
}

pub struct Call {
    token: Token,
    children: Vec<Box<dyn Node>>,
    type_id: NodeType,
    result_addr: String,
}

impl Call {

    fn get_child(&mut self, no: usize) -> Option<&mut Box<dyn Node>> {
        match self.children.get_mut(no) {
            Some(child) => Some(child),
            None => None,
        }
    }

    pub fn get_arguments(&mut self) -> Option<&mut Box<dyn Node>> {
        self.get_child(0)
    }


}

impl Node for Call {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_call(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, type_id: NodeType) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> NodeType {
        self.type_id.clone()
    }
    fn set_result_addr(&mut self, addr: String) {
        self.result_addr = addr;
    }
    fn get_result_addr(&self) -> String {
        self.result_addr.clone()
    }
}

pub struct IfNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl IfNode {
    pub fn get_condition(&mut self) -> Option<&mut Box<dyn Node>> {
        self.children.get_mut(0)
    }
    pub fn get_body(&mut self) -> Option<&mut Box<dyn Node>> {
        self.children.get_mut(1)
    }
    pub fn get_else_body(&mut self) -> Option<&mut Box<dyn Node>> {
        self.children.get_mut(2)
    }
}

impl Node for IfNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_if(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }

    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub struct WhileNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl WhileNode {
    pub fn get_condition(&mut self) -> Option<&mut Box<dyn Node>> {
        self.children.get_mut(0)
    }
    pub fn get_body(&mut self) -> Option<&mut Box<dyn Node>> {
        self.children.get_mut(1)
    }
}

impl Node for WhileNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_while(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }

    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub struct ParameterNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for ParameterNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub struct ArgumentNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for ArgumentNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_argument(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }
    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub struct ErrorNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for ErrorNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.insert(self.children.len(), child);
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_type(&mut self, _type_id: NodeType) {
        ()
    }
    fn get_type(&self) -> NodeType {
        NodeType::Unit
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }

    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub fn make_node(token: Token, flag: &str) -> Box<dyn Node> {
    match token.token_kind {
        TokenKind::Program => Box::from(Program {
            token,
            children: Vec::new(),
        }),
        TokenKind::Begin => Box::from(Block {
            token,
            children: Vec::new(),
            scope_no: -1,
        }),
        TokenKind::Var => Box::from(Declaration {
            token,
            children: Vec::new(),
        }),
        TokenKind::Assign => Box::from(Assignment {
            token,
            children: Vec::new(),
        }),
        TokenKind::Identifier => match flag {
            "var" => Box::from(Variable {
                token,
                children: Vec::new(),
                type_id: NodeType::Unit,
                result_addr: String::new(),
            }),
            "call" => Box::from(Call {
                token,
                children: Vec::new(),
                type_id: NodeType::Unit,
                result_addr: String::new(),
            }),
            _ => Box::from(Identifier {
                token,
                children: Vec::new(),
            }),
        },
        TokenKind::RealLiteral | TokenKind::StringLiteral | TokenKind::IntegerLiteral => {
            Box::from(Literal {
                token,
                children: Vec::new(),
                type_id: NodeType::Unit,
                result_addr: String::new(),
            })
        }
        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Multi
        | TokenKind::Division
        | TokenKind::Modulo
        | TokenKind::SmallerThan
        | TokenKind::LargerThan
        | TokenKind::ESmallerThan
        | TokenKind::ELargerThan
        | TokenKind::Equal
        | TokenKind::Or
        | TokenKind::And => Box::from(Expression {
            token,
            children: Vec::new(),
            type_id: NodeType::Unit,
            result_addr: String::new(),
        }),
        TokenKind::If => Box::from(IfNode {
            token,
            children: Vec::new(),
        }),
        TokenKind::While => Box::from(WhileNode {
            token,
            children: Vec::new(),
        }),
        TokenKind::OpenBracket => match flag {
            "params" => Box::from(ParameterNode{token, children: Vec::new() }),
            "args" => Box::from(ArgumentNode{token, children: Vec::new()}),
            _ => Box::from(ErrorNode{token, children:Vec::new()}),
        },
        _ => Box::from(ErrorNode {
            token: token,
            children: Vec::new(),
        }),
    }
}

pub fn get_args_node(token: Token) -> Box<dyn Node> {
    Box::from(ArgumentNode{token, children: Vec::new()})
}
