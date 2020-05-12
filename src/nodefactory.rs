use crate::ast::*;
use crate::scanner::Token;
use crate::scanner::TokenKind;

pub enum Flags {
    Params,
    Args,
    Call,
    Var,
    Def,
}

pub struct NodeFactory {
    token: Option<Token>,
    children: Vec<Box<dyn Node>>,
    flag: Flags,
}

impl NodeFactory {
    pub fn new() -> NodeFactory {
        NodeFactory {
            token: None,
            children: Vec::new(),
            flag: Flags::Def,
        }
    }

    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.flag = flag;
    }

    pub fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }

    fn unbox_node_to_identifier(maybenode: Option<&Box<dyn Node>>) -> Option<&Identifier> {
        match maybenode {
            Some(node) => match node.as_any().downcast_ref::<&Identifier>() {
                Some(id) => Some(id),
                None => None,
            },
            None => None,
        }
    }

    fn unbox_node_to_variable(maybenode: Option<&Box<dyn Node>>) -> Option<&Variable> {
        match maybenode {
            Some(node) => match node.as_any().downcast_ref::<Variable>() {
                Some(id) => Some(id),
                None => None,
            },
            None => None,
        }
    }

    fn build_program(&self, token: Token) -> Option<Box<dyn Node>> {
        let maybenode = NodeFactory::unbox_node_to_identifier(self.children.get(0));
        if let Some(id_child) = maybenode {
            Some(Box::from(Program {
                token,
                children: self.children,
                id_child: *id_child,
            }))
        } else {
            None
        }
    }

    fn build_declaration(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(id_child) = NodeFactory::unbox_node_to_identifier(self.children.get(0)) {
            if let Some(type_child) = NodeFactory::unbox_node_to_identifier(self.children.get(1)) {
                Some(Box::from(Declaration {
                    token,
                    children: self.children,
                    id_child: *id_child,
                    type_child: *type_child,
                }))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn build_assignment(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(lhs_child) = NodeFactory::unbox_node_to_variable(self.children.get(0)) {
            if let Some(rhs_child) = self.children.get(1) {
                Some(Box::from(Assignment {
                    token,
                    children: self.children,
                    rhs_child: *rhs_child,
                    lhs_child: *lhs_child,
                }))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn build_expression(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(lhs_child) = self.children.get(0) {
            if let Some(rhs_child) = self.children.get(1) {
                Some(Box::from(Expression {
                    token,
                    children: self.children,
                    rhs_child: Box::from(*rhs_child),
                    lhs_child: Box::from(*lhs_child),
                    type_id: NodeType::Unit,
                    result_addr: String::new(),
                }))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn build_variable(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(index_child) = self.children.get(0) {
            Some(Box::from(Variable {
                token,
                children: self.children,
                index_child: Some(*index_child),
                result_addr: String::new(),
                type_id: NodeType::Unit,
            }))
        } else {
            Some(Box::from(Variable {
                token,
                children: self.children,
                index_child: None,
                result_addr: String::new(),
                type_id: NodeType::Unit,
            }))
        }
    }

    fn build_identifier(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(len_child) = self.children.get(0) {
            Some(Box::from(Identifier {
                token,
                children: self.children,
                type_id_len: Some(*len_child),
            }))
        } else {
            Some(Box::from(Identifier {
                token,
                children: self.children,
                type_id_len: None,
            }))
        }
    }

    fn build_literal(&self, token: Token) -> Option<Box<dyn Node>> {
        Some(Box::from(Literal {
            token,
            type_id: NodeType::Unit,
            result_addr: String::new(),
            children: Vec::new(),
        }))
    }

    fn build_call(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(args) = self.children.get(0) {
            Some(Box::from(Call {
                token,
                type_id: NodeType::Unit,
                result_addr: String::new(),
                arguments: *args,
                children: self.children,
            }))
        } else {
            None
        }
    }

    fn build_if(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(condition) = self.children.get(0) {
            if let Some(body) = self.children.get(1) {
                if let Some(else_body) = self.children.get(2) {
                    Some(Box::from(IfNode {
                        token,
                        children: self.children,
                        body: *body,
                        condition: *condition,
                        else_body: Some(*else_body),
                    }))
                } else {
                    Some(Box::from(IfNode {
                        token,
                        children: self.children,
                        body: *body,
                        condition: *condition,
                        else_body: None,
                    }))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn build_while(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(condition) = self.children.get(0) {
            if let Some(body) = self.children.get(1) {
                Some(Box::from(WhileNode {
                    token,
                    children: self.children,
                    body: *body,
                    condition: *condition,
                }))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn build_args(&self, token: Token) -> Option<Box<dyn Node>> {
        Some(Box::from(ArgumentNode {
            token,
            children: self.children,
        }))
    }

    fn build_params(&self, token: Token) -> Option<Box<dyn Node>> {
        Some(Box::from(ParametersNode {
            token,
            children: self.children,
        }))
    }

    fn build_assert(&self, token: Token) -> Option<Box<dyn Node>> {
        if let Some(condition) = self.children.get(0) {
            Some(Box::from(AssertNode {
                token,
                condition: *condition,
                children: self.children,
            }))
        } else {
            None
        }
    }

    fn build_function(&self, token: Token) -> Option<Box<dyn Node>> {
        None
    }

    pub fn build_node(&mut self) -> Option<Box<dyn Node>> {
        let children = self.children;
        match self.token {
            Some(token) => match token.token_kind {
                TokenKind::Program => self.build_program(token),
                TokenKind::Begin => Some(Box::from(Block {
                    token,
                    children,
                    scope_no: -1,
                })),
                TokenKind::Var => self.build_declaration(token),
                TokenKind::Assign => self.build_assignment(token),
                TokenKind::Identifier => match self.flag {
                    Flags::Var => self.build_variable(token),
                    Flags::Call => self.build_call(token),
                    _ => self.build_identifier(token),
                },
                TokenKind::RealLiteral | TokenKind::StringLiteral | TokenKind::IntegerLiteral => {
                    self.build_literal(token)
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
                | TokenKind::NotEqual
                | TokenKind::Or
                | TokenKind::And => self.build_expression(token),
                TokenKind::If => self.build_if(token),
                TokenKind::While => self.build_while(token),
                TokenKind::OpenBracket => match self.flag {
                    Flags::Params => self.build_params(token),
                    Flags::Args => self.build_args(token),
                    _ => None,
                },
                TokenKind::Assert => self.build_assert(token),
                TokenKind::Function => self.build_function(token),
                _ => None,
            },
            None => None,
        }
    }
}
