use crate::ast::*;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenKind;

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
    ctt: TokenKind,
    pub errors: Vec<String>,
}

impl Parser {
    fn next_token(&mut self) {
        self.current_token = self.scanner.get_next_token();
        self.ctt = self.current_token.token_kind;
    }

    fn skip_delimiter(&mut self, kind: TokenKind) -> Result<(), String> {
        if kind == self.ctt {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "Expected {} got {}",
                kind, self.current_token.token_kind
            ))
        }
    }

    fn handle_error(&mut self, msg: &str) {
        let line = self.current_token.row + 1;
        let column = self.current_token.column + 1;
        let complete_message = format!(
            "Syntax error: {} On line: {}, column: {}.",
            msg, line, column
        );
        self.errors.push(complete_message);
    }

    pub fn program(&mut self) -> Option<AST> {
        self.next_token();
        if let TokenKind::Program = self.ctt {
            self.next_token();
            if let TokenKind::Identifier = self.ctt {
                let id = self.current_token.clone();
                self.next_token();
                if let Err(msg) = self.skip_delimiter(TokenKind::SemiColon) {
                    self.handle_error(msg.as_str());
                    return None;
                }
                if let Some(subroutines) = self.functions_and_procedures() {
                    if let Some(main_block) = self.block() {
                        if let Statement::Block(block) = main_block {
                            return Some(AST::Program(id, subroutines, block));
                        }
                    }
                }
            }
        }
        self.handle_error("Program does not start with program");
        None
    }

    fn functions_and_procedures(&mut self) -> Option<Vec<Subroutine>> {
        let mut subroutines = Vec::new();
        loop {
            match self.current_token.token_kind {
                TokenKind::Function => {
                    if let Some(f) = self.function() {
                        subroutines.push(f);
                    }
                }
                TokenKind::Procedure => {
                    if let Some(p) = self.procedure() {
                        subroutines.push(p)
                    }
                }
                TokenKind::Eof => return None,
                _ => break,
            }
        }
        Some(subroutines)
    }

    fn parameters(&mut self) -> Option<Vec<(Token, TypeDescription)>> {
        let mut parameters = Vec::new();
        self.next_token();
        loop {
            match self.ctt {
                TokenKind::Identifier => {
                    let id = self.current_token.clone();
                    self.next_token();
                    if let Err(msg) = self.skip_delimiter(TokenKind::Colon) {
                        self.handle_error(msg.as_str());
                    } else {
                        if let Some(type_construct) = self.type_construct() {
                            let param = (id, type_construct);
                            parameters.push(param);
                        }
                    }
                }
                TokenKind::Comma => self.next_token(),
                TokenKind::CloseBracket => {
                    self.next_token();
                    break;
                }
                _ => break,
            }
        }
        Some(parameters)
    }

    fn function(&mut self) -> Option<Subroutine> {
        None
    }

    fn procedure(&mut self) -> Option<Subroutine> {
        self.next_token();
        if let TokenKind::Identifier = self.ctt {
            let token = self.current_token.clone();
            self.next_token();
            if let Some(parameters) = self.parameters() {
                if let Err(msg) = self.skip_delimiter(TokenKind::SemiColon) {
                    self.handle_error(msg.as_str());
                } else {
                    if let Some(block) = self.block() {
                        if let Statement::Block(is_block_ok) = block {
                            if let Err(msg) =
                                self.skip_delimiter(TokenKind::SemiColon)
                            {
                                self.handle_error(msg.as_str());
                            }
                            return Some(Subroutine::Procedure(
                                token,
                                parameters,
                                is_block_ok,
                            ));
                        }
                    }
                }
            }
        }
        return None;
    }

    fn block(&mut self) -> Option<Statement> {
        self.next_token();
        let mut statements = Vec::new();
        loop {
            match self.ctt {
                TokenKind::End => {
                    self.next_token();
                    break;
                }
                TokenKind::SemiColon => self.next_token(),
                TokenKind::Eof => {
                    self.handle_error("Unexpected eof");
                    return None;
                }
                _ => {
                    if let Some(statement) = self.statement() {
                        statements.push(statement);
                    } else {
                        self.next_token();
                    }
                }
            };
        }
        Some(Statement::Block(statements))
    }

    fn statement(&mut self) -> Option<Statement> {
        match self.current_token.token_kind {
            TokenKind::Var => self.declaration_stmnt(),
            TokenKind::Identifier => self.assign_or_call_stmnt(),
            TokenKind::If => self.if_stmnt(),
            TokenKind::While => self.while_stmnt(),
            TokenKind::Begin => self.block(),
            TokenKind::Return => self.return_stmnt(),
            TokenKind::Assert => self.assert_stmnt(),
            _ => {
                let text = format!(
                    "Statement can not start with {}",
                    self.current_token.lexeme.clone()
                );
                self.handle_error(text.as_str());
                None
            }
        }
    }

    fn while_stmnt(&mut self) -> Option<Statement> {
        if let TokenKind::While = self.ctt {
            self.next_token();
            if let Some(expression) = self.expression() {
                if let Err(msg) = self.skip_delimiter(TokenKind::Do) {
                    self.handle_error(msg.as_str());
                } else if let Some(statement) = self.statement() {
                    return Some(Statement::While(
                        expression,
                        Box::from(statement),
                    ));
                } else {
                    self.handle_error("Missing while body");
                }
            }
        }
        None
    }

    fn if_stmnt(&mut self) -> Option<Statement> {
        if let TokenKind::If = self.ctt {
            self.next_token();
            if let Some(expression) = self.expression() {
                if let Err(msg) = self.skip_delimiter(TokenKind::Then) {
                    self.handle_error(msg.as_str());
                    return None;
                } else if let Some(statement) = self.statement() {
                    if let TokenKind::Else = self.ctt {
                        self.next_token();
                        if let Some(else_statement) = self.statement() {
                            return Some(Statement::If(
                                expression,
                                Box::from(statement),
                                Some(Box::from(else_statement)),
                            ));
                        }
                        self.handle_error("Missing else body");
                        return None;
                    } else {
                        return Some(Statement::If(
                            expression,
                            Box::from(statement),
                            None,
                        ));
                    }
                }
            } else {
                self.handle_error("If with no condition");
            }
        }
        None
    }

    fn type_construct(&mut self) -> Option<TypeDescription> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let type_token = self.current_token.clone();
                self.next_token();
                Some(TypeDescription::Simple(type_token))
            }
            TokenKind::Array => {
                self.next_token();
                if let Err(msg) =
                    self.skip_delimiter(TokenKind::OpenSquareBracket)
                {
                    self.errors.push(msg);
                    None
                } else if let Some(expression) = self.expression() {
                    if let Err(msg) =
                        self.skip_delimiter(TokenKind::CloseSquareBracket)
                    {
                        self.errors.push(msg);
                        None
                    } else if let Err(msg) = self.skip_delimiter(TokenKind::Of)
                    {
                        self.errors.push(msg);
                        None
                    } else if let TokenKind::Identifier = self.ctt {
                        let type_token = self.current_token.clone();
                        self.next_token();
                        Some(TypeDescription::Array(type_token, expression))
                    } else {
                        self.handle_error("Missing simple type for array type");
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

    fn declaration_stmnt(&mut self) -> Option<Statement> {
        if let TokenKind::Var = self.ctt {
            self.next_token();
            if let TokenKind::Identifier = self.ctt {
                let id_token = self.current_token.clone();
                self.next_token();
                if let Err(msg) = self.skip_delimiter(TokenKind::Colon) {
                    self.handle_error(msg.as_str());
                    return None;
                } else {
                    if let Some(type_description) = self.type_construct() {
                        return Some(Statement::Declaration(
                            id_token,
                            type_description,
                        ));
                    }
                }
            }
        }
        None
    }

    fn assign_or_call_stmnt(&mut self) -> Option<Statement> {
        match self.variable_or_call() {
            None => None,
            Some(id_construct) => match self.current_token.token_kind {
                TokenKind::Assign => match id_construct {
                    Expression::Variable(variable) => {
                        self.next_token();
                        if let Some(expression) = self.expression() {
                            Some(Statement::Assign(variable, expression))
                        } else {
                            None
                        }
                    }
                    Expression::Call(_token, _params) => {
                        let msg = format!("Can't assign to a call.");
                        self.errors.push(msg);
                        None
                    }
                    _ => None,
                },
                _ => match id_construct {
                    Expression::Variable(_variable) => {
                        let msg = "Statement consisting of only a variable";
                        self.handle_error(msg);
                        None
                    }
                    Expression::Call(node, args) => {
                        Some(Statement::Call(node, args))
                    }
                    _ => None,
                },
            },
        }
    }

    fn assert_stmnt(&mut self) -> Option<Statement> {
        let main_token = self.current_token.clone();
        self.next_token();
        if let Err(msg) = self.skip_delimiter(TokenKind::OpenBracket) {
            self.handle_error(msg.as_str());
            return None;
        }
        if let Some(expression) = self.expression() {
            if let Err(msg) = self.skip_delimiter(TokenKind::CloseBracket) {
                self.handle_error(msg.as_str());
                None
            } else {
                Some(Statement::Assert(main_token, expression))
            }
        } else {
            let text = "Assert requires expression";
            self.handle_error(text);
            None
        }
    }

    fn return_stmnt(&mut self) -> Option<Statement> {
        if let TokenKind::Return = self.current_token.token_kind {
            let token = self.current_token.clone();
            self.next_token();
            let expression = if !(self.ctt == TokenKind::SemiColon) {
                self.expression()
            } else {
                None
            };
            Some(Statement::Return(token, expression))
        } else {
            None
        }
    }

    fn expression(&mut self) -> Option<Expression> {
        match self.simple_expression() {
            Some(left_sub_expr) => match self.current_token.token_kind {
                TokenKind::Equal
                | TokenKind::SmallerThan
                | TokenKind::NotEqual
                | TokenKind::LargerThan
                | TokenKind::ESmallerThan
                | TokenKind::ELargerThan => {
                    let expr_token = self.current_token.clone();
                    self.next_token();
                    if let Some(right_sub_expr) = self.simple_expression() {
                        Some(Expression::Binary(
                            Box::from(left_sub_expr),
                            Box::from(right_sub_expr),
                            expr_token,
                        ))
                    } else {
                        None
                    }
                }
                _ => Some(left_sub_expr),
            },
            None => None,
        }
    }

    fn simple_expression(&mut self) -> Option<Expression> {
        match self.term() {
            Some(left_term) => match self.current_token.token_kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Or => {
                    let op_token = self.current_token.clone();
                    self.next_token();
                    if let Some(right_term) = self.term() {
                        Some(Expression::Binary(
                            Box::from(left_term),
                            Box::from(right_term),
                            op_token,
                        ))
                    } else {
                        None
                    }
                }
                _ => Some(left_term),
            },
            None => None,
        }
    }

    fn term(&mut self) -> Option<Expression> {
        match self.factor() {
            Some(lhs_node) => match self.current_token.token_kind {
                TokenKind::Multi
                | TokenKind::Division
                | TokenKind::Modulo
                | TokenKind::And => {
                    let term_token = self.current_token.clone();
                    self.next_token();
                    if let Some(rhs_node) = self.factor() {
                        Some(Expression::Binary(
                            Box::from(lhs_node),
                            Box::from(rhs_node),
                            term_token,
                        ))
                    } else {
                        None
                    }
                }
                _ => Some(lhs_node),
            },
            None => None,
        }
    }

    fn factor(&mut self) -> Option<Expression> {
        match self.current_token.token_kind {
            TokenKind::Identifier => match self.variable_or_call() {
                Some(id_construct) => match id_construct {
                    Expression::Call(token, args) => {
                        Some(Expression::Call(token, args))
                    }
                    Expression::Variable(node) => {
                        Some(Expression::Variable(node))
                    }
                    _ => None,
                },
                None => None,
            },
            TokenKind::RealLiteral
            | TokenKind::StringLiteral
            | TokenKind::IntegerLiteral => {
                let token = self.current_token.clone();
                self.next_token();
                Some(Expression::Literal(token))
            }
            TokenKind::OpenBracket => {
                self.next_token();
                let node = match self.expression() {
                    Some(expr_node) => Some(expr_node),
                    None => None,
                };
                match self.skip_delimiter(TokenKind::CloseBracket) {
                    Ok(_delim) => node,
                    Err(msg) => {
                        self.handle_error(msg.as_str());
                        None
                    }
                }
            }
            TokenKind::Not => {
                let token = self.current_token.clone();
                self.next_token();
                if let Some(rhs_node) = self.factor() {
                    Some(Expression::Unary(Box::from(rhs_node), token))
                } else {
                    self.handle_error("Expected right hand side for unary");
                    None
                }
            }
            _ => {
                self.handle_error(
                    format!("Cant create factor from {}", self.ctt).as_str(),
                );
                self.next_token();
                None
            }
        }
    }

    fn variable_or_call(&mut self) -> Option<Expression> {
        match self.current_token.token_kind {
            TokenKind::Identifier => {
                let old_token = self.current_token.clone();
                self.next_token();
                match self.current_token.token_kind {
                    TokenKind::OpenSquareBracket => {
                        self.next_token();
                        if let Some(expr) = self.expression() {
                            if let Err(msg) = self
                                .skip_delimiter(TokenKind::CloseSquareBracket)
                            {
                                self.handle_error(msg.as_str());
                                None
                            } else {
                                Some(Expression::Variable(Box::from(
                                    Variable::Indexed(old_token, expr),
                                )))
                            }
                        } else {
                            self.handle_error("Missing array index expression");
                            None
                        }
                    }
                    TokenKind::OpenBracket => {
                        if let Some(args) = self.arguments() {
                            Some(Expression::Call(old_token, args))
                        } else {
                            None
                        }
                    }
                    TokenKind::Dot => {
                        self.next_token();
                        let args = vec![Expression::Variable(Box::from(
                            Variable::Simple(old_token),
                        ))];
                        if let TokenKind::Identifier =
                            self.current_token.token_kind
                        {
                            let id_token = self.current_token.clone();
                            self.next_token();
                            Some(Expression::Call(id_token, args))
                        } else {
                            None
                        }
                    }
                    _ => Some(Expression::Variable(Box::from(
                        Variable::Simple(old_token),
                    ))),
                }
            }
            _ => None,
        }
    }

    fn arguments(&mut self) -> Option<Vec<Expression>> {
        match self.current_token.token_kind {
            TokenKind::OpenBracket => {
                let mut arguments = Vec::new();
                self.next_token();
                match self.ctt {
                    TokenKind::CloseBracket => {
                        self.next_token();
                        Some(arguments)
                    }
                    _ => {
                        if let Some(expr) = self.expression() {
                            arguments.push(expr);
                            loop {
                                if let TokenKind::Comma = self.ctt {
                                    self.next_token();
                                    if let Some(expr) = self.expression() {
                                        arguments.push(expr)
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                        if let Err(msg) =
                            self.skip_delimiter(TokenKind::CloseBracket)
                        {
                            self.errors.push(msg)
                        }
                        Some(arguments)
                    }
                }
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
        ctt: TokenKind::Error,
    }
}
