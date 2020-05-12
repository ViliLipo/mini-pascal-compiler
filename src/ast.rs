use crate::scanner::Token;
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
                NodeType::Simple(s2) => {
                    if s.as_str() == "Any" || s2.as_str() == "Any" {
                        true
                    } else {
                        s == s2
                    }
                }
                _ => false,
            },
            NodeType::ArrayOf(s) => match other {
                NodeType::ArrayOf(s2) => {
                    if s.as_str() == "Any" || s2.as_str() == "Any" {
                        true
                    } else {
                        s == s2
                    }
                }
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
    id_child: Identifier,
}

impl Program {
    pub fn get_id_child(&self) -> &Identifier {
        &self.id_child
    }
}

impl Node for Program {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    id_child: Identifier,
    type_child: Identifier,
}

impl Declaration {
    pub fn get_id_child(&self) -> &Identifier {
        &self.id_child
    }
    pub fn get_type_child(&self) -> &Identifier {
        &self.type_child
    }
}

impl Node for Declaration {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    rhs_child: Box<dyn Node>,
    lhs_child: Variable,
}

impl Assignment {
    pub fn get_rhs_child(&self) -> &Box<dyn Node> {
        &self.rhs_child
    }
    pub fn get_lhs_child(&self) -> &Variable {
        &self.lhs_child
    }
}
impl Node for Assignment {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    rhs_child: Box<dyn Node>,
    lhs_child: Box<dyn Node>,
}

impl Expression {
    pub fn get_lhs_child(&self) -> &Box<dyn Node> {
        &self.lhs_child
    }
    pub fn get_rhs_child(&self) -> &Box<dyn Node> {
        &self.rhs_child
    }
}

impl Node for Expression {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    index_child: Option<Box<dyn Node>>,
}

impl Variable {
    pub fn get_index_child(&self) -> &Option<Box<dyn Node>> {
        &self.index_child
    }
}

impl Node for Variable {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    type_id_len: Option<Box<dyn Node>>,
}

// For some reason i decided to overload this node with the type information
impl Identifier {
    pub fn get_type_id_len_child(&self) -> &Option<Box<dyn Node>> {
        &self.type_id_len
    }
}

impl Node for Identifier {
    fn get_token(&self) -> Token {
        self.token.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    type_id: NodeType,
    result_addr: String,
    children: Vec<Box<dyn Node>>,
}

impl Node for Literal {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut Vec::new()
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
    arguments: Box<dyn Node>,
}

impl Call {
    fn get_child(&mut self, no: usize) -> Option<&mut Box<dyn Node>> {
        match self.children.get_mut(no) {
            Some(child) => Some(child),
            None => None,
        }
    }

    pub fn get_arguments(&mut self) -> &mut Box<dyn Node> {
        &mut self.arguments
    }
}

impl Node for Call {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    condition: Box<dyn Node>,
    body: Box<dyn Node>,
    else_body: Option<Box<dyn Node>>
}

impl IfNode {
    pub fn get_condition(&mut self) -> &mut Box<dyn Node> {
        &mut self.condition
    }
    pub fn get_body(&mut self) -> &mut Box<dyn Node> {
        &mut self.body
    }
    pub fn get_else_body(&mut self) -> &mut Option<Box<dyn Node>> {
        &mut self.else_body
    }
}

impl Node for IfNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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
    condition: Box<dyn Node>,
    body: Box<dyn Node>,
}

impl WhileNode {
    pub fn get_condition(&mut self) -> &mut Box<dyn Node> {
        &mut self.condition
    }
    pub fn get_body(&mut self) -> &mut Box<dyn Node> {
        &mut self.body
    }
}

impl Node for WhileNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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

pub struct ParametersNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for ParametersNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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

pub struct ParametersItemNode {
    token: Token,
    identifier: Box<dyn Node>,
    type_identifier: Box<dyn Node>,
    children: Vec<Box<dyn Node>>,
}

impl ParametersItemNode {
    pub fn get_identifier(&self) -> Box<dyn Node> {
        self.identifier
    }

    pub fn get_type(&self) -> Box<dyn Node> {
        self.type_identifier
    }
}

impl Node for ParametersItemNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
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

pub struct FunctionNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
}

impl Node for FunctionNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_function(self);
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

pub struct AssertNode {
    token: Token,
    children: Vec<Box<dyn Node>>,
    condition: Box<dyn Node>,
}

impl AssertNode {
    pub fn get_condition(&mut self) -> &mut Box<dyn Node> {
        &mut self.condition
    }
}

impl Node for AssertNode {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>> {
        &mut self.children
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_assert(self);
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


pub fn get_args_node(token: Token) -> Box<dyn Node> {
    Box::from(ArgumentNode {
        token,
        children: Vec::new(),
    })
}
