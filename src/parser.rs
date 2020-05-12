use crate::ast::*;
use crate::nodefactory::Flags;
use crate::nodefactory::NodeFactory;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenKind;

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
    pub errors: Vec<String>,
}

enum IdExpression {
    Variable(Box<dyn Node>),
    Call(Box<dyn Node>),
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = self.scanner.get_next_token();
    }

    fn skip_delimiter(&mut self, tokenkind: TokenKind) -> Result<(), String> {
        if self.current_token.token_kind == tokenkind {
            self.next_token();
            Ok(())
        } else {
            Err(String::from(format!(
                "Expected {} got {}",
                tokenkind, self.current_token.token_kind
            )))
        }
    }

    fn match_token(&mut self, tokenkind: TokenKind) -> Result<Box<dyn Node>, String> {
        if self.current_token.token_kind == tokenkind {
            let mut nodefactory = NodeFactory::new();
            nodefactory.set_token(self.current_token.clone());
            if let Some(node) = nodefactory.build_node() {
                self.next_token();
                Ok(node)
            } else {
                Err(String::from(format!("Could not build node")))
            }
        } else {
            Err(String::from(format!(
                "Expected {} got {}",
                tokenkind, self.current_token.token_kind
            )))
        }
    }

    pub fn program(&mut self) -> Option<Box<dyn Node>> {
        self.next_token();
        let mut nf = NodeFactory::new();
        if let TokenKind::Program = self.current_token.token_kind {
            nf.set_token(self.current_token.clone());
            match self.match_token(TokenKind::Identifier) {
                Ok(id) => nf.add_child(id),
                Err(msg) => {
                    self.errors.push(msg);
                    return None;
                }
            };
            if let Err(msg) = self.skip_delimiter(TokenKind::SemiColon) {
                self.errors.push(msg);
                return None;
            };
            match self.block() {
                Some(block) => nf.add_child(block),
                None => {
                    return None;
                }
            };
            if let Err(msg) = self.skip_delimiter(TokenKind::Dot) {
                self.errors.push(msg);
            };
            if let Some(main_node) = nf.build_node() {
                Some(main_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parameters(&mut self) -> Option<Box<dyn Node>> {
        let mut nf = NodeFactory::new();
        nf.set_flag(Flags::Params);
        nf.set_token(self.current_token.clone());
        self.next_token();
        loop {
            if let Ok(identifier) = self.match_token(TokenKind::Identifier) {
                match self.match_token(TokenKind::Colon) {
                    Ok(_delim) => match self.type_construct() {
                        Some(type_identifier) => {
                            let item = ParametersItemNode {
                                identifier,
                                type_identifier,
                                token: self.current_token.clone(),
                                children: Vec::new(),
                            };
                            nf.add_child(Box::from(item));
                            if let Err(msg) = self.match_token(TokenKind::Comma) {
                                self.errors.push(msg);
                            }
                        }
                        _ => (),
                    },
                    Err(msg) => self.errors.push(msg),
                };
            } else {
                break;
            }
        }
        if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
            self.errors.push(msg);
        }
        nf.build_node()
    }

    fn function(&mut self) -> Option<Box<dyn Node>> {
        None
        /*
        if let TokenKind::Function = self.current_token.token_kind {
            let mut main_node = make_node(self.current_token.clone(), "");
            self.next_token();
            if let TokenKind::Identifier = self.current_token.token_kind {
                let id_child = make_node(self.current_token.clone(), "");
                main_node.add_child(id_child);
                self.next_token();
                if let Err(msg) = self.match_token(TokenKind::OpenBracket) {
                    self.errors.push(msg);
                    None
                } else {
                    if let Some(param_child) = self.parameters() {
                        main_node.add_child(param_child);
                    }
                    if let Err(msg) = self.match_token(TokenKind::Colon) {
                        self.errors.push(msg);
                        None
                    } else {
                        if let TokenKind::Identifier = self.current_token.token_kind {
                            let type_child = make_node(self.current_token.clone(), "");
                            main_node.add_child(type_child);
                            if let Err(msg) = self.match_token(TokenKind::SemiColon) {
                                self.errors.push(msg);
                                None
                            } else {
                                if let Some(block) = self.block() {
                                    main_node.add_child(block);
                                    if let Err(msg) = self.match_token(TokenKind::SemiColon) {
                                        self.errors.push(msg);
                                        None
                                    } else {
                                        Some(main_node)
                                    }
                                } else {
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                }
            } else {
                let row = self.current_token.row;
                let text = format!("Function with no name on line {}.", row);
                self.errors.push(text);
                None
            }
        } else {
            None
        }*/
    }

    fn procedure(&mut self) -> Option<Box<dyn Node>> {
        None
    }

    fn block(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::Begin = self.current_token.token_kind {
            let mut nf = NodeFactory::new();
            nf.set_token(self.current_token.clone());
            self.next_token();
            loop {
                match self.current_token.token_kind {
                    TokenKind::End => {
                        self.next_token();
                        break;
                    }
                    TokenKind::SemiColon => self.next_token(),
                    TokenKind::Eof => {
                        self.errors.push(String::from("Unexpected eof"));
                        break;
                    }
                    _ => match self.statement() {
                        Some(stmnt) => nf.add_child(stmnt),
                        None => self.next_token(),
                    },
                }
            }
            nf.build_node()
        } else {
            None
        }
    }

    fn statement(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Var => self.declaration_stmnt(),
            TokenKind::Identifier => self.assign_or_call_stmnt(),
            TokenKind::If => self.if_stmnt(),
            TokenKind::While => self.while_stmnt(),
            TokenKind::Begin => self.block(),
            TokenKind::Assert => self.assert_stmnt(),
            _ => None,
        }
    }

    fn while_stmnt(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::While = self.current_token.token_kind {
            let mut nf = NodeFactory::new();
            nf.set_token(self.current_token.clone());
            self.next_token();
            if let Some(condition) = self.expression() {
                nf.add_child(condition);
                if let Err(msg) = self.skip_delimiter(TokenKind::Do) {
                    self.errors.push(msg);
                    None
                } else {
                    if let Some(body) = self.statement() {
                        nf.add_child(body);
                        nf.build_node()
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn if_stmnt(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::If = self.current_token.token_kind {
            let mut nf = NodeFactory::new();
            nf.set_token(self.current_token.clone());
            self.next_token();
            if let Some(condition) = self.expression() {
                nf.add_child(condition);
                if let Err(msg) = self.skip_delimiter(TokenKind::Then) {
                    self.errors.push(msg);
                    return None;
                }
                if let Some(body) = self.statement() {
                    nf.add_child(body);
                    if let Err(msg) = self.skip_delimiter(TokenKind::Then) {
                        self.errors.push(msg);
                        return None;
                    }
                    if let Some(else_stmnt) = self.statement() {
                        nf.add_child(else_stmnt);
                    }
                    return nf.build_node();
                }
            }
        }
        None
    }

    fn type_construct(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let mut nf = NodeFactory::new();
                nf.set_token(self.current_token.clone());
                self.next_token();
                nf.build_node()
            }
            TokenKind::Array => {
                self.next_token();
                if let Err(msg) = self.match_token(TokenKind::OpenSquareBracket) {
                    self.errors.push(msg);
                    return None;
                }
                if let Some(expr_node) = self.expression() {
                    if let Err(msg) = self.match_token(TokenKind::CloseSquareBracket) {
                        self.errors.push(msg);
                        None
                    } else if let Err(msg) = self.skip_delimiter(TokenKind::Of) {
                        self.errors.push(msg);
                        return None;
                    } else if let TokenKind::Identifier = self.current_token.token_kind {
                        let mut nf = NodeFactory::new();
                        nf.add_child(expr_node);
                        nf.set_token(self.current_token.clone());
                        nf.build_node()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => {
                self.errors.push(String::from("No type given"));
                None
            }
        }
    }

    fn declaration_stmnt(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::Var = self.current_token.token_kind {
            let mut nf = NodeFactory::new();
            nf.set_token(self.current_token.clone());
            self.next_token();
            match self.match_token(TokenKind::Identifier) {
                Ok(id) => {
                    nf.add_child(id);
                    if let Err(msg) = self.skip_delimiter(TokenKind::Colon) {
                        self.errors.push(msg);
                        None
                    } else {
                        if let Some(type_node) = self.type_construct() {
                            nf.add_child(type_node);
                            nf.build_node()
                        } else {
                            None
                        }
                    }
                }
                Err(msg) => {
                    self.errors.push(msg);
                    None
                }
            }
        } else {
            None
        }
    }

    fn assign_or_call_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.variable_or_call() {
            None => None,
            Some(id_construct) => match self.current_token.token_kind {
                TokenKind::Assign => match id_construct {
                    IdExpression::Variable(node) => {
                        let mut nf = NodeFactory::new();
                        nf.set_token(self.current_token.clone());
                        self.next_token();
                        nf.add_child(node);
                        match self.expression() {
                            Some(expr_node) => {
                                nf.add_child(expr_node);
                                nf.build_node()
                            }
                            None => None,
                        }
                    }
                    IdExpression::Call(_node) => {
                        let msg = format!("Can't assign to a call.");
                        self.errors.push(msg);
                        None
                    }
                },
                _ => match id_construct {
                    IdExpression::Variable(node) => {
                        let row = node.get_token().row;
                        println!("{}", node.get_token().lexeme);
                        let msg =
                            format!("Statement consisting of only a variable on line {}", row);
                        println!("{}", msg);
                        self.errors.push(msg);
                        None
                    }
                    IdExpression::Call(node) => Some(node),
                },
            },
        }
    }

    fn assert_stmnt(&mut self) -> Option<Box<dyn Node>> {
        let mut nf = NodeFactory::new();
        nf.set_token(self.current_token.clone());
        self.next_token();
        match self.match_token(TokenKind::OpenBracket) {
            Ok(_delim) => {
                if let Some(expr_node) = self.expression() {
                    nf.add_child(expr_node);
                    if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
                        self.errors.push(msg);
                    }
                    nf.build_node()
                } else {
                    let row = self.current_token.row;
                    let text = format!("Assert requires expression on line: {}", row);
                    self.errors.push(text);
                    None
                }
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn expression(&mut self) -> Option<Box<dyn Node>> {
        match self.simple_expression() {
            Some(left_sub_expr) => match self.current_token.token_kind {
                TokenKind::Equal
                | TokenKind::SmallerThan
                | TokenKind::NotEqual
                | TokenKind::LargerThan
                | TokenKind::ESmallerThan
                | TokenKind::ELargerThan => {
                    let mut nf = NodeFactory::new();
                    nf.set_token(self.current_token.clone());
                    self.next_token();
                    nf.add_child(left_sub_expr);
                    match self.simple_expression() {
                        Some(right_sub_expr) => {
                            nf.add_child(right_sub_expr);
                            nf.build_node()
                        }
                        None => None,
                    }
                }
                _ => Some(left_sub_expr),
            },
            None => None,
        }
    }

    fn simple_expression(&mut self) -> Option<Box<dyn Node>> {
        match self.term() {
            Some(term_node) => match self.current_token.token_kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Or => {
                    let mut nf = NodeFactory::new();
                    nf.set_token(self.current_token.clone());
                    self.next_token();
                    nf.add_child(term_node);
                    match self.term() {
                        Some(right_term_node) => {
                            nf.add_child(right_term_node);
                            nf.build_node()
                        }
                        None => None,
                    }
                }
                _ => Some(term_node),
            },
            None => None,
        }
    }

    fn term(&mut self) -> Option<Box<dyn Node>> {
        match self.factor() {
            Some(lhs_node) => match self.current_token.token_kind {
                TokenKind::Multi | TokenKind::Division | TokenKind::Modulo | TokenKind::And => {
                    let mut nf = NodeFactory::new();
                    nf.set_token(self.current_token.clone());
                    self.next_token();
                    nf.add_child(lhs_node);
                    match self.factor() {
                        Some(rhs_node) => {
                            nf.add_child(rhs_node);
                            nf.build_node()
                        }
                        None => None,
                    }
                }
                _ => Some(lhs_node),
            },
            None => None,
        }
    }

    fn factor(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Identifier => match self.variable_or_call() {
                Some(id_construct) => match id_construct {
                    IdExpression::Call(node) => Some(node),
                    IdExpression::Variable(node) => Some(node),
                },
                None => None,
            },
            TokenKind::RealLiteral | TokenKind::StringLiteral | TokenKind::IntegerLiteral => {
                let mut nf = NodeFactory::new();
                nf.set_token(self.current_token.clone());
                self.next_token();
                nf.build_node()
            }
            TokenKind::OpenBracket => {
                self.next_token();
                let node = match self.expression() {
                    Some(expr_node) => Some(expr_node),
                    None => None,
                };
                match self.skip_delimiter(TokenKind::CloseBracket) {
                    Ok(i) => (),
                    Err(msg) => self.errors.push(msg),
                }
                node
            }
            _ => None,
        }
    }

    fn variable_or_call(&mut self) -> Option<IdExpression> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let old_token = self.current_token.clone();
                self.next_token();
                match self.current_token.token_kind {
                    TokenKind::OpenSquareBracket => {
                        let mut nf = NodeFactory::new();
                        nf.set_token(old_token);
                        nf.set_flag(Flags::Var);
                        self.next_token();
                        if let Some(expr) = self.expression() {
                            nf.add_child(expr);
                        }
                        match self.match_token(TokenKind::CloseSquareBracket) {
                            Ok(_delim) => (),
                            Err(msg) => self.errors.push(msg),
                        }
                        if let Some(node) = nf.build_node() {
                            Some(IdExpression::Variable(node))
                        } else {
                            None
                        }
                    }
                    TokenKind::OpenBracket => {
                        let mut nf = NodeFactory::new();
                        nf.set_token(old_token);
                        nf.set_flag(Flags::Call);
                        if let Some(args) = self.arguments() {
                            nf.add_child(args);
                        }
                        if let Some(node) = nf.build_node() {
                            Some(IdExpression::Call(node))
                        } else {
                            None
                        }
                    }
                    TokenKind::Dot => {
                        self.next_token();
                        let mut nf1 = NodeFactory::new();
                        nf1.set_token(old_token);
                        nf1.set_flag(Flags::Var);
                        if let Some(arg) = nf1.build_node() {
                            if let TokenKind::Identifier = self.current_token.token_kind {
                                let nf2 = NodeFactory::new();
                                nf2.set_token(self.current_token.clone());
                                nf2.set_flag(Flags::Call);
                                self.next_token();
                                let args = ArgumentNode {
                                    token: self.current_token.clone(),
                                    children: vec![arg],
                                };
                                nf2.add_child(Box::from(args));
                                if let Some(node) = nf2.build_node() {
                                    Some(IdExpression::Call(node))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => {
                        let mut nf = NodeFactory::new();
                        nf.set_flag(Flags::Var);
                        nf.set_token(old_token.clone());
                        if let Some(node) = nf.build_node() {
                            Some(IdExpression::Variable(node))
                        } else {
                            None
                        }
                    }
                }
            }
            _ => None,
        }
    }

    fn arguments(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::OpenBracket => {
                let mut nf = NodeFactory::new();
                nf.set_token(self.current_token.clone());
                nf.set_flag(Flags::Args);
                self.next_token();
                if let Some(expr) = self.expression() {
                    nf.add_child(expr);
                    loop {
                        if let TokenKind::Comma = self.current_token.token_kind {
                            self.next_token();
                            if let Some(expr) = self.expression() {
                                nf.add_child(expr)
                            }
                        } else {
                            break;
                        }
                    }
                }
                if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
                    self.errors.push(msg)
                }
                nf.build_node()
            }
            _ => None,
        }
    }
}

pub fn build_parser(scanner: Scanner) -> Parser {
    Parser {
        scanner: scanner,
        errors: Vec::new(),
        current_token: Token {
            token_kind: TokenKind::Error,
            lexeme: String::from(""),
            column: 0,
            row: 0,
        },
    }
}
