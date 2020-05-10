use crate::ast::make_node;
use crate::ast::get_args_node;
use crate::ast::Node;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenKind;

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = self.scanner.get_next_token();
    }

    fn match_token(&mut self, tokenkind: TokenKind) -> Result<Box<dyn Node>, String> {
        if self.current_token.token_kind == tokenkind {
            let node = make_node(self.current_token.clone(), "");
            self.next_token();
            Ok(node)
        } else {
            Err(String::from(format!(
                "Expected {} got {}",
                tokenkind, self.current_token.token_kind
            )))
        }
    }

    pub fn program(&mut self) -> Option<Box<dyn Node>> {
        self.next_token();
        match self.match_token(TokenKind::Program) {
            Ok(mut main_node) => {
                match self.match_token(TokenKind::Identifier) {
                    Ok(id) => main_node.add_child(id),
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                }
                match self.match_token(TokenKind::SemiColon) {
                    Ok(_delim) => (),
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                }
                match self.block() {
                    Some(block) => main_node.add_child(block),
                    None => {
                        return None;
                    }
                };
                match self.match_token(TokenKind::Dot) {
                    Ok(_node) => (),
                    Err(msg) => self.errors.push(msg),
                };
                Some(main_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn block(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token(TokenKind::Begin) {
            Ok(mut main_node) => {
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
                            Some(stmnt) => main_node.add_child(stmnt),
                            None => self.next_token(),
                        },
                    }
                }
                Some(main_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn statement(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Var => self.declaration_stmnt(),
            TokenKind::Identifier => self.assign_or_call_stmnt(),
            TokenKind::If => self.if_stmnt(),
            TokenKind::While => self.while_stmnt(),
            TokenKind::Begin => self.block(),
            _ => None,
        }
    }

    fn while_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token(TokenKind::While) {
            Ok(mut while_node) => {
                if let Some(expr_node) = self.expression() {
                    while_node.add_child(expr_node);
                    if let Ok(_do) = self.match_token(TokenKind::Do) {
                        if let Some(body) = self.statement() {
                            while_node.add_child(body);
                        }
                    }
                }
                Some(while_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn if_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token(TokenKind::If) {
            Ok(mut if_node) => {
                if let Some(expr_node) = self.expression() {
                    if_node.add_child(expr_node);
                    if let Ok(_then) = self.match_token(TokenKind::Then) {
                        if let Some(stmnt) = self.statement() {
                            if_node.add_child(stmnt);
                        }
                        match self.current_token.token_kind {
                            TokenKind::Else => {
                                self.next_token();
                                if let Some(else_stmnt) = self.statement() {
                                    if_node.add_child(else_stmnt);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Some(if_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn declaration_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token(TokenKind::Var) {
            Ok(mut main_node) => {
                match self.match_token(TokenKind::Identifier) {
                    Ok(id_node) => {
                        main_node.add_child(id_node);
                    }
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                };
                match self.match_token(TokenKind::Colon) {
                    Ok(_delim) => (),
                    Err(msg) => self.errors.push(msg),
                };
                match self.current_token.token_kind {
                    TokenKind::Identifier => {
                        let type_node = make_node(self.current_token.clone(), "");
                        main_node.add_child(type_node);
                    }
                    TokenKind::Array => {
                        self.next_token();
                        if let TokenKind::OpenSquareBracket = self.current_token.token_kind {
                            self.next_token();
                            if let Some(expr) = self.expression() {
                                match self.match_token(TokenKind::CloseSquareBracket) {
                                    Ok(_delim) => match self.match_token(TokenKind::Of) {
                                        Ok(_delim) => {
                                            match self.match_token(TokenKind::Identifier) {
                                                Ok(id_node) => {
                                                    main_node.add_child(id_node);
                                                    main_node.add_child(expr);
                                                }
                                                Err(msg) => self.errors.push(msg),
                                            }
                                        }
                                        Err(msg) => self.errors.push(msg),
                                    },
                                    Err(msg) => self.errors.push(msg),
                                }
                            }
                        } else {
                            self.errors.push(format!("No size found in array type"));
                        }
                    }
                    _ => self.errors.push(String::from("No type given")),
                }
                Some(main_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn assign_or_call_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.variable_or_call() {
            None => None,
            Some(node) => match self.current_token.token_kind {
                TokenKind::Assign => { // TODO: FORBID assign to a call
                    let mut main_node = make_node(self.current_token.clone(), "");
                    self.next_token();
                    main_node.add_child(node);
                    match self.expression() {
                        Some(expr_node) => {
                            main_node.add_child(expr_node);
                            Some(main_node)
                        }
                        None => None,
                    }
                }
                _ => Some(node),
            },
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
                    let mut expr_node = make_node(self.current_token.clone(), "");
                    self.next_token();
                    expr_node.add_child(left_sub_expr);
                    match self.simple_expression() {
                        Some(right_sub_expr) => {
                            expr_node.add_child(right_sub_expr);
                            Some(expr_node)
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
                    let mut expr_node = make_node(self.current_token.clone(), "");
                    self.next_token();
                    expr_node.add_child(term_node);
                    match self.term() {
                        Some(right_term_node) => {
                            expr_node.add_child(right_term_node);
                            Some(expr_node)
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
                    let mut term_node = make_node(self.current_token.clone(), "");
                    self.next_token();
                    term_node.add_child(lhs_node);
                    match self.factor() {
                        Some(rhs_node) => {
                            term_node.add_child(rhs_node);
                            Some(term_node)
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
            TokenKind::Identifier => self.variable_or_call(),
            TokenKind::RealLiteral | TokenKind::StringLiteral | TokenKind::IntegerLiteral => {
                let node = make_node(self.current_token.clone(), "");
                self.next_token();
                Some(node)
            }
            TokenKind::OpenBracket => {
                self.next_token();
                let node = match self.expression() {
                    Some(expr_node) => Some(expr_node),
                    None => None,
                };
                match self.match_token(TokenKind::CloseBracket) {
                    Ok(_delim) => (),
                    Err(msg) => self.errors.push(msg),
                }
                node
            }
            _ => None,
        }
    }

    fn variable_or_call(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let old_token = self.current_token.clone();
                self.next_token();
                match self.current_token.token_kind {
                    TokenKind::OpenSquareBracket => {
                        let mut node = make_node(old_token, "var");
                        self.next_token();
                        if let Some(expr) = self.expression() {
                            node.add_child(expr);
                        }
                        match self.match_token(TokenKind::CloseSquareBracket) {
                            Ok(_delim) => (),
                            Err(msg) => self.errors.push(msg),
                        }
                        Some(node)
                    },
                    TokenKind::OpenBracket => {
                        let mut node = make_node(old_token, "call");
                        if let Some(args) = self.arguments() {
                            node.add_child(args);
                        }
                        Some(node)
                    },
                    TokenKind::Dot => {
                        self.next_token();
                        let arg = make_node(old_token, "var");
                        if let TokenKind::Identifier = self.current_token.token_kind{
                            let mut node = make_node(self.current_token.clone(), "call");
                            self.next_token();
                            let mut args = get_args_node(self.current_token.clone());
                            args.add_child(arg);
                            node.add_child(Box::from(args));
                            Some(node)
                        } else {
                            None
                        }
                    },
                    _ => Some(make_node(old_token, "var")),
                }
            }
            _ => None,
        }
    }

    fn arguments(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::OpenBracket => {
                let mut main_node = make_node(self.current_token.clone(), "args");
                self.next_token();
                if let Some(expr) = self.expression() {
                    main_node.add_child(expr);
                    println!("Added expr child");
                    loop {
                        if let TokenKind::Comma = self.current_token.token_kind {
                            self.next_token();
                            if let Some(expr) = self.expression() {
                                main_node.add_child(expr)
                            }
                        } else {
                            break;
                        }
                    }
                }
                if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
                    self.errors.push(msg)
                }
                println!("Parsed arguments");
                Some(main_node)
            },
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
