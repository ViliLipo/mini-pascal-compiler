use crate::ast::make_node;
use crate::ast::Node;
use crate::scanner::Scanner;
use crate::scanner::Token;

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = self.scanner.get_next_token();
    }

    fn match_token(&mut self, tokenkind: &str) -> Result<Box<dyn Node>, String> {
        if self.current_token.t_type.as_str() == tokenkind {
            let node = make_node(self.current_token.clone(), "");
            self.next_token();
            Ok(node)
        } else {
            Err(String::from(format!(
                "Expected {} got {}",
                tokenkind, self.current_token.t_type
            )))
        }
    }

    pub fn program(&mut self) -> Option<Box<dyn Node>> {
        self.next_token();
        match self.match_token("program") {
            Ok(mut main_node) => {
                match self.match_token("identifier") {
                    Ok(id) => main_node.add_child(id),
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                }
                match self.match_token(";") {
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
                match self.match_token(".") {
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
        match self.match_token("begin") {
            Ok(mut main_node) => {
                loop {
                    match self.current_token.t_type.as_str() {
                        "end" => {
                            self.next_token();
                            break;
                        }
                        ";" => self.next_token(),
                        "eof" => {
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
        match self.current_token.t_type.as_str() {
            "var" => self.declaration_stmnt(),
            "identifier" => self.assign_or_call_stmnt(),
            "if" => self.if_stmnt(),
            "while" => self.while_stmnt(),
            "begin" => self.block(),
            _ => None,
        }
    }

    fn while_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token("while") {
            Ok(mut while_node) => {
                if let Some(expr_node) = self.expression() {
                    while_node.add_child(expr_node);
                    if let Ok(_do) = self.match_token("do") {
                        if let Some(body) = self.statement() {
                            while_node.add_child(body);
                        }
                    }
                }
                Some(while_node)
            },
            Err(msg) => {
                self.errors.push(msg);
                None
            },
        }
    }

    fn if_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token("if") {
            Ok(mut if_node) => {
                if let Some(expr_node) = self.expression() {
                    if_node.add_child(expr_node);
                    if let Ok(_then) = self.match_token("then") {
                        if let Some(stmnt) = self.statement() {
                            if_node.add_child(stmnt);
                        }
                        match self.current_token.t_type.as_str() {
                            "else" => {
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
        match self.match_token("var") {
            Ok(mut main_node) => {
                match self.match_token("identifier") {
                    Ok(id_node) => {
                        main_node.add_child(id_node);
                    }
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                };
                match self.match_token(":") {
                    Ok(_delim) => (),
                    Err(msg) => self.errors.push(msg),
                };
                match self.match_token("identifier") {
                    Ok(type_node) => {
                        main_node.add_child(type_node);
                    }
                    Err(msg) => {
                        self.errors.push(msg);
                        return None;
                    }
                };
                Some(main_node)
            }
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }

    fn assign_or_call_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token("identifier") {
            Err(msg) => {
                self.errors.push(msg);
                None
            }
            Ok(id_node) => match self.current_token.t_type.as_str() {
                ":=" => {
                    let mut main_node = make_node(self.current_token.clone(), "");
                    self.next_token();
                    main_node.add_child(id_node);
                    match self.expression() {
                        Some(expr_node) => {
                            main_node.add_child(expr_node);
                            Some(main_node)
                        }
                        None => None,
                    }
                }
                "(" => {
                    self.next_token();
                    let mut call_node = make_node(id_node.get_token(), "call");
                    match self.arguments() {
                        Some(args_node) => {
                            call_node.add_child(args_node);
                            match self.match_token(")") {
                                Ok(_delim) => Some(call_node),
                                Err(msg) => {
                                    self.errors.push(msg);
                                    None
                                }
                            }
                        }
                        None => Some(call_node),
                    }
                }
                _ => None,
            },
        }
    }

    fn expression(&mut self) -> Option<Box<dyn Node>> {
        match self.simple_expression() {
            Some(left_sub_expr) => match self.current_token.t_type.as_str() {
                "=" | "<" | "<>" | ">" | "<=" | ">=" => {
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
        // TODO: Sign and add addition operator types
        match self.term() {
            Some(term_node) => match self.current_token.t_type.as_str() {
                "+" | "-" => {
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
            Some(lhs_node) => match self.current_token.t_type.as_str() {
                "*" | "/" => {
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
        match self.current_token.t_type.as_str() {
            "identifier" | "real_literal" | "string_literal" | "integer_literal" => {
                let node = make_node(self.current_token.clone(), "");
                self.next_token();
                Some(node)
            }
            "(" => {
                self.next_token();
                let node = match self.expression() {
                    Some(expr_node) => Some(expr_node),
                    None => None,
                };
                match self.match_token(")") {
                    Ok(_delim) => (),
                    Err(msg) => self.errors.push(msg),
                }
                node
            }
            _ => None,
        }
    }

    fn arguments(&mut self) -> Option<Box<dyn Node>> {
        match self.expression() {
            Some(expr_node) => Some(expr_node),
            None => None,
        }
    }

    fn write_stmnt(&mut self) -> Option<Box<dyn Node>> {
        match self.match_token("writeln") {
            Ok(mut main_node) => match self.match_token("(") {
                Ok(_delim) => match self.expression() {
                    Some(expr_node) => {
                        main_node.add_child(expr_node);
                        match self.match_token("(") {
                            Ok(_delim) => Some(main_node),
                            Err(msg) => {
                                self.errors.push(msg);
                                None
                            }
                        }
                    }
                    None => None,
                },
                Err(msg) => {
                    self.errors.push(msg);
                    None
                }
            },
            Err(msg) => {
                self.errors.push(msg);
                None
            }
        }
    }
}

pub fn build_parser(scanner: Scanner) -> Parser {
    Parser {
        scanner: scanner,
        errors: Vec::new(),
        current_token: Token {
            t_type: String::from(""),
            lexeme: String::from(""),
            column: 0,
            row: 0,
        },
    }
}
