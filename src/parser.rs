use crate::ast::make_node;
use crate::ast::Node;
use crate::ast::*;
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

    fn match_token(&mut self, tokenkind: TokenKind) -> Result<Box<dyn Node>, String> {
        if self.current_token.token_kind == tokenkind {
            let node = make_node(self.current_token.clone(), "");
            self.next_token();
            Ok(node)
        } else {
            let line = self.current_token.row + 1;
            let column = self.current_token.column + 1;
            Err(String::from(format!(
                "Expected {} got {}, on line: {}, column: {},",
                tokenkind, self.current_token.token_kind, line, column
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
                if let Some(subroutines) = self.functions_and_procedures() {
                    main_node.add_child(subroutines);
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

    fn functions_and_procedures(&mut self) -> Option<Box<dyn Node>> {
        let mut node = SubRoutineList::new(self.current_token.clone());
        loop {
            match self.current_token.token_kind {
                TokenKind::Function => {
                    if let Some(f) = self.function() {
                        node.add_child(f)
                    }
                }
                TokenKind::Procedure => {
                    if let Some(p) = self.procedure() {
                        node.add_child(p)
                    }
                }
                _ => break,
            }
        }
        Some(Box::from(node))
    }

    fn parameters(&mut self) -> Option<Box<dyn Node>> {
        let mut parameters_node = make_node(self.current_token.clone(), "params");
        self.next_token();
        loop {
            if let Ok(identifier) = self.match_token(TokenKind::Identifier) {
                match self.match_token(TokenKind::Colon) {
                    Ok(_delim) => match self.type_construct() {
                        Some(type_identifier) => {
                            let mut item = ParameterItemNode::new(self.current_token.clone());
                            item.add_child(identifier);
                            item.add_child(type_identifier);
                            parameters_node.add_child(Box::from(item));
                            if let Err(_msg) = self.match_token(TokenKind::Comma) {
                                break;
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
        Some(parameters_node)
    }

    fn function(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::Function = self.current_token.token_kind {
            let mut main_node = make_node(self.current_token.clone(), "");
            self.next_token();
            if let TokenKind::Identifier = self.current_token.token_kind {
                let id_child = make_node(self.current_token.clone(), "");
                main_node.add_child(id_child);
                self.next_token();
                if let Some(param_child) = self.parameters() {
                    main_node.add_child(param_child);
                }
                if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
                    self.errors.push(msg);
                    None
                } else {
                    if let Err(msg) = self.match_token(TokenKind::Colon) {
                        self.errors.push(msg);
                        None
                    } else {
                        if let Some(type_child) = self.type_construct() {
                            main_node.add_child(type_child);
                            if let Err(msg) = self.match_token(TokenKind::SemiColon) {
                                self.errors.push(msg);
                                None
                            } else {
                                println!("wow: {}", self.current_token.token_kind);
                                if let Some(block) = self.block() {
                                    main_node.add_child(block);
                                    if let Err(msg) = self.match_token(TokenKind::SemiColon) {
                                        self.errors.push(msg);
                                    }
                                    Some(main_node)
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
        }
    }

    fn procedure(&mut self) -> Option<Box<dyn Node>> {
        None
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
        println!("lel: {}", self.current_token.token_kind);
        match self.current_token.token_kind {
            TokenKind::Var => self.declaration_stmnt(),
            TokenKind::Identifier => self.assign_or_call_stmnt(),
            TokenKind::If => self.if_stmnt(),
            TokenKind::While => self.while_stmnt(),
            TokenKind::Begin => self.block(),
            TokenKind::Return => self.return_stmnt(),
            TokenKind::Assert => self.assert_stmnt(),
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

    fn type_construct(&mut self) -> Option<Box<dyn Node>> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let type_node = make_node(self.current_token.clone(), "");
                self.next_token();
                Some(type_node)
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
                    } else if let Err(msg) = self.match_token(TokenKind::Of) {
                        self.errors.push(msg);
                        return None;
                    } else if let Ok(mut id_node) = self.match_token(TokenKind::Identifier) {
                        id_node.add_child(expr_node);
                        Some(id_node)
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
                if let Some(type_node) = self.type_construct() {
                    main_node.add_child(type_node);
                    Some(main_node)
                } else {
                    None
                }
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
            Some(id_construct) => match self.current_token.token_kind {
                TokenKind::Assign => match id_construct {
                    IdExpression::Variable(node) => {
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
        let mut main_node = make_node(self.current_token.clone(), "");
        self.next_token();
        match self.match_token(TokenKind::OpenBracket) {
            Ok(_delim) => {
                if let Some(expr_node) = self.expression() {
                    main_node.add_child(expr_node);
                    if let Err(msg) = self.match_token(TokenKind::CloseBracket) {
                        self.errors.push(msg);
                    }
                    Some(main_node)
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

    fn return_stmnt(&mut self) -> Option<Box<dyn Node>> {
        if let TokenKind::Return = self.current_token.token_kind {
            let mut node = make_node(self.current_token.clone(), "");
            self.next_token();
            if let Some(expr_child) = self.expression() {
                node.add_child(expr_child);
            }
            Some(node)
        } else {
            None
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
            TokenKind::Identifier => match self.variable_or_call() {
                Some(id_construct) => match id_construct {
                    IdExpression::Call(node) => Some(node),
                    IdExpression::Variable(node) => Some(node),
                },
                None => None,
            },
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

    fn variable_or_call(&mut self) -> Option<IdExpression> {
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
                        Some(IdExpression::Variable(node))
                    }
                    TokenKind::OpenBracket => {
                        let mut node = make_node(old_token, "call");
                        if let Some(args) = self.arguments() {
                            node.add_child(args);
                        }
                        Some(IdExpression::Call(node))
                    }
                    TokenKind::Dot => {
                        self.next_token();
                        let arg = make_node(old_token, "var");
                        if let TokenKind::Identifier = self.current_token.token_kind {
                            let mut node = make_node(self.current_token.clone(), "call");
                            self.next_token();
                            let mut args = get_args_node(self.current_token.clone());
                            args.add_child(arg);
                            node.add_child(Box::from(args));
                            Some(IdExpression::Call(node))
                        } else {
                            None
                        }
                    }
                    _ => {
                        let node = make_node(old_token, "var");
                        Some(IdExpression::Variable(node))
                    }
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
