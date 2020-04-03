use crate::scanner::Token;
use crate::visitor::Visitor;
use std::any::Any;

pub trait Node {
    fn get_token(&self) -> Token;
    fn get_children(&mut self) -> &mut Vec<Box<dyn Node>>;
    fn add_child(&mut self, child: Box<dyn Node>);
    fn accept(&mut self, visitor: &mut dyn Visitor);
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> String;
    fn set_type(&mut self, type_id: String);
    fn get_result_addr(&self) -> String;
    fn set_result_addr(&mut self, address: String);
}

pub struct Program {
    token: Token,
    children: Vec<Box<dyn Node>>,
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("integer")
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
    pub fn get_type_child(&self) -> Option<&Variable> {
        self.get_child(1)
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("")
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("")
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("")
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
    type_id: String,
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
    fn set_type(&mut self, type_id: String) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> String {
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
    type_id: String,
    result_addr: String,
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
    fn set_type(&mut self, type_id: String) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> String {
        self.type_id.clone()
    }
    fn set_result_addr(&mut self, addr: String) {
        self.result_addr = addr;
    }
    fn get_result_addr(&self) -> String {
        self.result_addr.clone()
    }
}

pub struct Literal {
    token: Token,
    children: Vec<Box<dyn Node>>,
    type_id: String,
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
    fn set_type(&mut self, type_id: String) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> String {
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
    type_id: String,
    result_addr: String,
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
    fn set_type(&mut self, type_id: String) {
        self.type_id = type_id;
    }
    fn get_type(&self) -> String {
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("If statement")
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("If statement")
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
    fn set_type(&mut self, _type_id: String) {
        ()
    }
    fn get_type(&self) -> String {
        String::from("Error")
    }
    fn set_result_addr(&mut self, _addr: String) {
        ()
    }

    fn get_result_addr(&self) -> String {
        String::new()
    }
}

pub fn make_node(token: Token, flag: &str) -> Box<dyn Node> {
    match token.t_type.as_str() {
        "program" => Box::from(Program {
            token,
            children: Vec::new(),
        }),
        "begin" => Box::from(Block {
            token,
            children: Vec::new(),
            scope_no: -1,
        }),
        "var" => Box::from(Declaration {
            token,
            children: Vec::new(),
        }),
        ":=" => Box::from(Assignment {
            token,
            children: Vec::new(),
        }),
        "identifier" => {
            if flag != "call" {
                Box::from(Variable {
                    token,
                    children: Vec::new(),
                    type_id: String::new(),
                    result_addr: String::new(),
                })
            } else {
                println!("Creating a call");
                Box::from(Call {
                    token,
                    children: Vec::new(),
                    type_id: String::new(),
                    result_addr: String::new(),
                })
            }
        }
        "real_literal" | "string_literal" | "integer_literal" => Box::from(Literal {
            token,
            children: Vec::new(),
            type_id: String::new(),
            result_addr: String::new(),
        }),
        "+" | "-" | "*" | "/" | "<" | ">" | "<=" | ">=" | "=" => Box::from(Expression {
            token,
            children: Vec::new(),
            type_id: String::new(),
            result_addr: String::new(),
        }),
        "if" => Box::from(IfNode {
            token,
            children: Vec::new(),
        }),
        "while" => Box::from(WhileNode {
            token,
            children: Vec::new(),
        }),
        _ => Box::from(ErrorNode {
            token: token,
            children: Vec::new(),
        }),
    }
}
