use crate::ast::*;
use crate::scanner::Token;
use crate::scanner::TokenKind;
use crate::symboltable::*;

pub enum OpKind {
    Addition,
    NumArithmetic,
    Modulo,
    Relational,
    BoolArithmetic,
    E,
}

fn string_as_opkind(op: &String) -> OpKind {
    match op.as_str() {
        "+" => OpKind::Addition,
        "-" | "/" | "*" => OpKind::NumArithmetic,
        "%" => OpKind::Modulo,
        "=" | "<>" | "<" | "<=" | ">=" | ">" => OpKind::Relational,
        "or" | "and" => OpKind::BoolArithmetic,
        _ => OpKind::E,
    }
}

fn type_id_to_simple_type(identifier: &String) -> Option<SimpleType> {
    match identifier.as_str() {
        "string" => Some(SimpleType::String),
        "integer" => Some(SimpleType::Integer),
        "Boolean" => Some(SimpleType::Boolean),
        "real" => Some(SimpleType::Real),
        _ => None,
    }
}

pub struct TypeFolder {
    errors: Vec<String>,
}

impl TypeFolder {
    pub fn new() -> TypeFolder {
        TypeFolder { errors: Vec::new() }
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }
    pub fn fold_ast(
        &mut self,
        node: &AST,
        st: &mut Symboltable,
    ) -> Option<TypedAST> {
        match node {
            AST::Program(token, subroutines, main_block) => {
                let mut typed_subroutines = Vec::new();
                for subroutine in subroutines {
                    if let Some(sr) = self.fold_subroutine(subroutine, st) {
                        typed_subroutines.push(sr);
                    }
                }
                if let TypedStatement::Block(block) =
                    self.fold_block(main_block, st)
                {
                    Some(TypedAST::Program(
                        token.clone(),
                        typed_subroutines,
                        block,
                    ))
                } else {
                    None
                }
            }
        }
    }

    fn fold_subroutine(
        &mut self,
        node: &Subroutine,
        st: &mut Symboltable,
    ) -> Option<TypedSubroutine> {
        None
    } // TODO SUBROUTINES

    fn fold_block(
        &mut self,
        node: &Vec<Statement>,
        st: &mut Symboltable,
    ) -> TypedStatement {
        let mut typed_statemens = Vec::new();
        st.new_scope_in_current_scope(false);
        for statement in node {
            if let Some(stmnt) = self.fold_statement(&statement, st) {
                typed_statemens.push(stmnt);
            }
        }
        st.exit_scope();
        TypedStatement::Block(typed_statemens)
    }

    fn fold_statement(
        &mut self,
        node: &Statement,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        match node {
            Statement::Assign(var, ex) => self.fold_assign(var, ex, st),
            Statement::Declaration(token, type_description) => {
                self.fold_declaration(token, type_description, st)
            }
            Statement::While(condition, body) => {
                self.fold_while(condition, body, st)
            }
            Statement::If(condition, body, maybe_else_body) => {
                if let Some(else_body) = maybe_else_body {
                    self.fold_if(condition, body, Some(else_body), st)
                } else {
                    self.fold_if(condition, body, None, st)
                }
            }
            Statement::Block(block) => Some(self.fold_block(block, st)),
            Statement::Assert(token, expression) => {
                self.fold_assert(token, expression, st)
            }
            Statement::Call(id, params) => {
                self.fold_call_statement(id, params, st)
            }
            Statement::Return(token, value) => {
                self.fold_return(token, value, st)
            }
        }
    }

    fn fold_assign(
        &mut self,
        variable: &Variable,
        value: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        if let Some(target) = self.fold_variable(variable, st) {
            if let Some(value) = self.fold_expression(value, st) {
                if self.match_variable_type_to_expression(&target, &value) {
                    return Some(TypedStatement::Assign(target, value));
                } else {
                    self.errors.push(format!("Mismatched types in assigment"));
                }
            }
        }
        None
    }

    fn match_variable_type_to_expression(
        &mut self,
        target: &TypedVariable,
        value: &TypedExpression,
    ) -> bool {
        match target {
            TypedVariable::Simple(simple_target_type, _addr) => match value {
                TypedExpression::Simple(simple_value_type, _addr) => self
                    .match_simple_typed_variable_to_expression(
                        simple_target_type,
                        simple_value_type,
                    ),
                _ => false,
            },
            TypedVariable::ArrayOf(array_target_type, _addr) => match value {
                TypedExpression::ArrayOf(array_value_type, _addr) => self
                    .match_simple_typed_variable_to_expression(
                        array_target_type,
                        array_value_type,
                    ),
                _ => false,
            },
        }
    }

    fn match_simple_typed_variable_to_expression(
        &mut self,
        target: &SimplyTypedVariable,
        value: &SimplyTypedExpression,
    ) -> bool {
        match target {
            SimplyTypedVariable::Int(_var) => match value {
                SimplyTypedExpression::Int(_expr) => true,
                _ => false,
            },
            SimplyTypedVariable::Boolean(_var) => match value {
                SimplyTypedExpression::Boolean(_expr) => true,
                _ => false,
            },
            SimplyTypedVariable::String(_var) => match value {
                SimplyTypedExpression::String(_expr) => true,
                _ => false,
            },
            SimplyTypedVariable::Real(_var) => match value {
                SimplyTypedExpression::Real(_expr) => true,
                _ => false,
            },
        }
    }

    fn fold_declaration(
        &mut self,
        token: &Token,
        type_description: &TypeDescription,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        let name = token.lexeme.clone();
        if let None = st.lookup(&name) {
            match type_description {
                TypeDescription::Simple(t) => {
                    self.fold_simple_declaration(token, t, st)
                }
                TypeDescription::Array(t, e) => {
                    self.fold_array_declaration(token, t, e, st)
                }
            }
        } else {
            self.errors.push(format!("Variable declared twice"));
            None
        }
    }

    fn fold_simple_declaration(
        &mut self,
        name_token: &Token,
        type_token: &Token,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        let type_identifier = type_token.lexeme.clone();
        if let Some(entry_base_type) = type_id_to_simple_type(&type_identifier)
        {
            let entry_type = NodeType::Simple(entry_base_type);
            let name = name_token.lexeme.clone();
            if self.is_valid_type(type_token, st) {
                let entry = Entry {
                    name,
                    category: ConstructCategory::SimpleVar,
                    scope_number: st.get_current_scope_number(),
                    entry_type,
                    address: String::new(),
                    value: String::new(),
                };
                st.add_entry(entry);
                Some(TypedStatement::Declaration(
                    name_token.clone(),
                    TypedTypeDescription::Simple(type_token.clone()),
                ))
            } else {
                self.errors.push(format!("Usage of an undeclared type"));
                None
            }
        } else {
            self.errors.push(format!("Usage of an unimplented type"));
            None
        }
    }

    fn fold_array_declaration(
        &mut self,
        name_token: &Token,
        type_token: &Token,
        type_expression: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        let maybe_size_expression = self.fold_expression(&type_expression, st);
        match maybe_size_expression {
            Some(typed_expr) => {
                if let TypedExpression::Simple(expr, _addr) = typed_expr {
                    if let SimplyTypedExpression::Int(simple_expr) = expr {
                        if let Some(base_entry_type) =
                            type_id_to_simple_type(&type_token.lexeme)
                        {
                            let entry_type = NodeType::ArrayOf(base_entry_type);
                            let entry = Entry {
                                name: name_token.lexeme.clone(),
                                category: ConstructCategory::ArrayVar,
                                entry_type,
                                scope_number: st.get_current_scope_number(),
                                address: String::new(),
                                value: String::new(),
                            };
                            st.add_entry(entry);
                            Some(TypedStatement::Declaration(
                                name_token.clone(),
                                TypedTypeDescription::Array(
                                    type_token.clone(),
                                    TypedExpression::Simple(
                                        SimplyTypedExpression::Int(simple_expr),
                                        0,
                                    ),
                                ),
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    self.errors.push(format!(
                        "Array lenght must be an integer expression"
                    ));
                    None
                }
            }
            None => {
                self.errors.push(format!("Array must have length"));
                None
            }
        }
    }

    fn is_valid_type(
        &mut self,
        type_token: &Token,
        st: &mut Symboltable,
    ) -> bool {
        if let Some(declared_type) = st.lookup(&type_token.lexeme) {
            if let ConstructCategory::TypeId = declared_type.category {
                true
            } else {
                self.errors.push(format!("Id does not refer to a type"));
                false
            }
        } else {
            self.errors.push(format!("Usage of an undeclared type"));
            false
        }
    }

    fn fold_if(
        &mut self,
        condition: &Expression,
        body: &Statement,
        else_body: Option<&Statement>,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        if let Some(typed_condition) = self.fold_expression(condition, st) {
            if let Some(typed_body) = self.fold_statement(body, st) {
                if let TypedExpression::Simple(simple_expr, _addr) =
                    typed_condition
                {
                    if let SimplyTypedExpression::Boolean(bool_expr) =
                        simple_expr
                    {
                        if let Some(eb) = else_body {
                            if let Some(typed_eb) = self.fold_statement(eb, st)
                            {
                                return Some(TypedStatement::If(
                                    SimplyTypedExpression::Boolean(bool_expr),
                                    Box::from(typed_body),
                                    Some(Box::from(typed_eb)),
                                ));
                            }
                        } else {
                            return Some(TypedStatement::If(
                                SimplyTypedExpression::Boolean(bool_expr),
                                Box::from(typed_body),
                                None,
                            ));
                        }
                    } else {
                        self.errors.push(format!(
                            "Non boolean exprssion in while statement"
                        ));
                    }
                } else {
                    self.errors.push(format!(
                        "Non simple expression as condition in while statement"
                    ));
                }
            }
        }
        None
    }

    fn fold_while(
        &mut self,
        condition: &Expression,
        body: &Statement,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        if let Some(typed_condition) = self.fold_expression(condition, st) {
            if let Some(typed_body) = self.fold_statement(body, st) {
                if let TypedExpression::Simple(simple_expr, _addr) =
                    typed_condition
                {
                    if let SimplyTypedExpression::Boolean(bool_expr) =
                        simple_expr
                    {
                        return Some(TypedStatement::While(
                            SimplyTypedExpression::Boolean(bool_expr),
                            Box::from(typed_body),
                        ));
                    } else {
                        self.errors.push(format!(
                            "Non boolean exprssion in while statement"
                        ));
                    }
                } else {
                    self.errors.push(format!(
                        "Non simple expression as condition in while statement"
                    ));
                }
            }
        }
        None
    }
    fn fold_assert(
        &mut self,
        token: &Token,
        condition: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        if let Some(typed_expression) = self.fold_expression(condition, st) {
            if let TypedExpression::Simple(simple_expr, _addr) =
                typed_expression
            {
                if let SimplyTypedExpression::Boolean(bool_expr) = simple_expr {
                    return Some(TypedStatement::Assert(
                        token.clone(),
                        SimplyTypedExpression::Boolean(bool_expr),
                    ));
                } else {
                    self.errors
                        .push(format!("Assert with non boolean condition"));
                }
            } else {
                self.errors
                    .push(format!("Assert with non array expression"));
            }
        }
        None
    }

    fn fold_call_expression(
        &mut self,
        token: &Token,
        arguments: &Vec<Expression>,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        let mut typedargs = Vec::new();
        for arg in arguments {
            if let Some(typedarg) = self.fold_expression(arg, st) {
                typedargs.push(typedarg)
            } else {
                self.errors.push(format!("Failed to check argument"));
            }
        }
        if let Some(entry) = st.lookup(&token.lexeme) {
            match &entry.category {
                ConstructCategory::Special => {
                    self.fold_special_call_expression(token, entry, typedargs)
                }
                ConstructCategory::Function(parameters, out_type) => self
                    .fold_regular_call_expression(
                        token, parameters, out_type, typedargs,
                    ),
                _ => None,
            }
        } else {
            None
        }
    }

    fn fold_special_call_expression(
        &mut self,
        token: &Token,
        entry: &Entry,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedExpression> {
        match entry.name.as_str() {
            "size" => {
                if let Some(argument) = arguments.get(0) {
                    if let TypedExpression::ArrayOf(_array_argument, _addr) =
                        argument
                    {
                        Some(TypedExpression::Simple(
                            SimplyTypedExpression::Int(
                                TypedExpressionCore::Call(
                                    token.clone(),
                                    arguments,
                                ),
                            ),
                            0,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn fold_regular_call_expression(
        &mut self,
        token: &Token,
        parameters: &Vec<NodeType>,
        out_type: &NodeType,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedExpression> {
        if self.match_arguments_to_parameters(&arguments, parameters) {
            let core = TypedExpressionCore::Call(token.clone(), arguments);
            match out_type {
                NodeType::Simple(st) => {
                    let st_expr = match st {
                        SimpleType::Integer => SimplyTypedExpression::Int(core),
                        SimpleType::Boolean => {
                            SimplyTypedExpression::Boolean(core)
                        }
                        SimpleType::Real => SimplyTypedExpression::Real(core),
                        SimpleType::String => {
                            SimplyTypedExpression::String(core)
                        }
                    };
                    Some(TypedExpression::Simple(st_expr, 0))
                }
                NodeType::ArrayOf(at) => {
                    let at_expr = match at {
                        SimpleType::Integer => SimplyTypedExpression::Int(core),
                        SimpleType::Boolean => {
                            SimplyTypedExpression::Boolean(core)
                        }
                        SimpleType::Real => SimplyTypedExpression::Real(core),
                        SimpleType::String => {
                            SimplyTypedExpression::String(core)
                        }
                    };
                    Some(TypedExpression::ArrayOf(at_expr, 0))
                }
            }
        } else {
            self.errors
                .push(format!("parameters do not match arguments"));
            None
        }
    }

    fn match_arguments_to_parameters(
        &mut self,
        arguments: &Vec<TypedExpression>,
        parameters: &Vec<NodeType>,
    ) -> bool {
        for i in 0..parameters.len() {
            if let Some(param) = parameters.get(i) {
                if let Some(arg) = arguments.get(i) {
                    if !self.match_argument_to_parameter(arg, param) {
                        return false;
                    }
                } else {
                    self.errors.push(format!("Too short argument list"));
                }
            }
        }
        true
    }

    fn match_argument_to_parameter(
        &mut self,
        argument: &TypedExpression,
        parameter: &NodeType,
    ) -> bool {
        match parameter {
            NodeType::Simple(simple_nodetype) => match argument {
                TypedExpression::Simple(simple_expr, _addr) => self
                    .match_simple_type_to_expression(
                        simple_nodetype,
                        simple_expr,
                    ),
                _ => false,
            },
            NodeType::ArrayOf(simple_nodetype) => match argument {
                TypedExpression::ArrayOf(simple_expr, _addr) => self
                    .match_simple_type_to_expression(
                        simple_nodetype,
                        simple_expr,
                    ),
                _ => false,
            },
        }
    }

    fn match_simple_type_to_expression(
        &mut self,
        simple_nodetype: &SimpleType,
        simple_expr: &SimplyTypedExpression,
    ) -> bool {
        match simple_nodetype {
            SimpleType::Boolean => match simple_expr {
                SimplyTypedExpression::Boolean(_expr) => true,
                _ => false,
            },
            SimpleType::Integer => match simple_expr {
                SimplyTypedExpression::Int(_expr) => true,
                _ => false,
            },
            SimpleType::Real => match simple_expr {
                SimplyTypedExpression::Real(_expr) => true,
                _ => false,
            },
            SimpleType::String => match simple_expr {
                SimplyTypedExpression::String(_expr) => true,
                _ => false,
            },
        }
    }

    fn fold_call_statement(
        &mut self,
        token: &Token,
        arguments: &Vec<Expression>,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        let mut typedargs = Vec::new();
        for arg in arguments {
            if let Some(typedarg) = self.fold_expression(arg, st) {
                typedargs.push(typedarg)
            } else {
                self.errors.push(format!("Failed to check argument"));
            }
        }
        if let Some(entry) = st.lookup(&token.lexeme) {
            match &entry.category {
                ConstructCategory::Special => {
                    self.fold_special_call_stmnt(token, entry, typedargs)
                }
                ConstructCategory::Procedure(parameters) => {
                    self.fold_regular_call(token, parameters, typedargs)
                }
                ConstructCategory::Function(_args, _params) => {
                    self.errors.push(format!(
                        "Attempting to call Function as statement"
                    ));
                    None
                }
                _ => {
                    self.errors.push(format!("Call of non subroutine name"));
                    None
                }
            }
        } else {
            self.errors.push(format!("Call of undeclared name"));
            None
        }
    }

    fn fold_special_call_stmnt(
        &mut self,
        token: &Token,
        entry: &Entry,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedStatement> {
        match entry.name.as_str() {
            "read" => None, // all args must be variables
            "writeln" => Some(TypedStatement::Call(token.clone(), arguments)), // all simple types go
            _ => None,
        }
    }

    fn fold_regular_call(
        &mut self,
        token: &Token,
        parameters: &Vec<NodeType>,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedStatement> {
        if self.match_arguments_to_parameters(&arguments, parameters) {
            Some(TypedStatement::Call(token.clone(), arguments))
        } else {
            None
        }
    }

    fn fold_return(
        &mut self,
        token: &Token,
        value: &Option<Expression>,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        if let Some(val) = value {
            if let Some(expr) = self.fold_expression(val, st) {
                Some(TypedStatement::Return(token.clone(), Some(expr)))
            } else {
                None
            }
        } else {
            Some(TypedStatement::Return(token.clone(), None))
        }
    }

    fn fold_expression(
        &mut self,
        node: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        match node {
            Expression::Literal(t) => self.fold_literal(t),
            Expression::Variable(v) => self.fold_variable_as_expression(v, st),
            Expression::Binary(lhs, rhs, op) => {
                self.fold_binary_expression(lhs, rhs, op, st)
            }
            Expression::Unary(rhs, op) => {
                self.fold_unary_expression(op, rhs, st)
            }
            Expression::Call(id, parameters) => {
                self.fold_call_expression(id, parameters, st)
            }
        }
    }

    fn fold_literal(&mut self, token: &Token) -> Option<TypedExpression> {
        match token.token_kind {
            TokenKind::IntegerLiteral => Some(TypedExpression::Simple(
                SimplyTypedExpression::Int(TypedExpressionCore::Literal(
                    token.clone(),
                )),
                0,
            )),
            TokenKind::StringLiteral => Some(TypedExpression::Simple(
                SimplyTypedExpression::String(TypedExpressionCore::Literal(
                    token.clone(),
                )),
                0,
            )),
            TokenKind::RealLiteral => Some(TypedExpression::Simple(
                SimplyTypedExpression::Real(TypedExpressionCore::Literal(
                    token.clone(),
                )),
                0,
            )),
            _ => {
                self.errors.push(format!("Non literal parsed as one"));
                None
            }
        }
    }

    fn fold_variable_as_expression(
        &mut self,
        var: &Variable,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        if let Some(variable) = self.fold_variable(var, st) {
            match variable {
                TypedVariable::Simple(tvar, _addr) => {
                    let expression = match tvar {
                        SimplyTypedVariable::Int(st) => {
                            SimplyTypedExpression::Int(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::Simple(
                                        SimplyTypedVariable::Int(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::Real(st) => {
                            SimplyTypedExpression::Real(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::Simple(
                                        SimplyTypedVariable::Real(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::Boolean(st) => {
                            SimplyTypedExpression::Boolean(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::Simple(
                                        SimplyTypedVariable::Boolean(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::String(st) => {
                            SimplyTypedExpression::String(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::Simple(
                                        SimplyTypedVariable::String(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                    };
                    Some(TypedExpression::Simple(expression, 0))
                }
                TypedVariable::ArrayOf(tvar, _addr) => {
                    let expression = match tvar {
                        SimplyTypedVariable::Int(st) => {
                            SimplyTypedExpression::Int(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::ArrayOf(
                                        SimplyTypedVariable::Int(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::Real(st) => {
                            SimplyTypedExpression::Real(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::ArrayOf(
                                        SimplyTypedVariable::Real(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::Boolean(st) => {
                            SimplyTypedExpression::Boolean(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::ArrayOf(
                                        SimplyTypedVariable::Boolean(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                        SimplyTypedVariable::String(st) => {
                            SimplyTypedExpression::String(
                                TypedExpressionCore::Variable(Box::from(
                                    TypedVariable::ArrayOf(
                                        SimplyTypedVariable::String(st),
                                        0,
                                    ),
                                )),
                            )
                        }
                    };
                    Some(TypedExpression::ArrayOf(expression, 0))
                }
            }
        } else {
            None
        }
    }

    fn fold_variable(
        &mut self,
        var: &Variable,
        st: &mut Symboltable,
    ) -> Option<TypedVariable> {
        match var {
            Variable::Simple(t) => self.fold_simple_variable(t, st),
            Variable::Indexed(t, e) => self.fold_indexed_variable(t, e, st),
        }
    }

    fn fold_simple_variable(
        &mut self,
        token: &Token,
        st: &Symboltable,
    ) -> Option<TypedVariable> {
        if let Some(entry) = st.lookup(&token.lexeme.clone()) {
            match &entry.entry_type {
                NodeType::Simple(t) => {
                    let variable = TypedVariableCore::Simple(token.clone());
                    let stv = match t {
                        SimpleType::Integer => {
                            SimplyTypedVariable::Int(variable)
                        }
                        SimpleType::Boolean => {
                            SimplyTypedVariable::Boolean(variable)
                        }
                        SimpleType::String => {
                            SimplyTypedVariable::String(variable)
                        }
                        SimpleType::Real => SimplyTypedVariable::Real(variable),
                    };
                    Some(TypedVariable::Simple(stv, 0))
                }
                NodeType::ArrayOf(t) => {
                    let variable = TypedVariableCore::Simple(token.clone());
                    let stv = match t {
                        SimpleType::Integer => {
                            SimplyTypedVariable::Int(variable)
                        }
                        SimpleType::Boolean => {
                            SimplyTypedVariable::Boolean(variable)
                        }
                        SimpleType::String => {
                            SimplyTypedVariable::String(variable)
                        }
                        SimpleType::Real => SimplyTypedVariable::Real(variable),
                    };
                    Some(TypedVariable::ArrayOf(stv, 0))
                }
            }
        } else {
            None
        }
    }

    fn fold_indexed_variable(
        &mut self,
        token: &Token,
        expression: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedVariable> {
        if let Some(typed_expression) = self.fold_expression(expression, st) {
            if let TypedExpression::Simple(simple_expr, _addr) =
                typed_expression
            {
                if let SimplyTypedExpression::Int(core_expr) = simple_expr {
                    if let Some(entry) = st.lookup(&token.lexeme) {
                        match &entry.entry_type {
                            NodeType::ArrayOf(simple_type) => {
                                let core = TypedVariableCore::Indexed(
                                    token.clone(),
                                    TypedExpression::Simple(
                                        SimplyTypedExpression::Int(core_expr),
                                        0,
                                    ),
                                );
                                let simply_typed = match simple_type {
                                    SimpleType::Integer => {
                                        SimplyTypedVariable::Int(core)
                                    }
                                    SimpleType::Real => {
                                        SimplyTypedVariable::Real(core)
                                    }
                                    SimpleType::Boolean => {
                                        SimplyTypedVariable::Boolean(core)
                                    }
                                    SimpleType::String => {
                                        SimplyTypedVariable::String(core)
                                    }
                                };
                                return Some(TypedVariable::Simple(
                                    simply_typed,
                                    0,
                                ));
                            }
                            NodeType::Simple(_st) => self
                                .errors
                                .push(format!("indexing simple variable")),
                        };
                    } else {
                        self.errors.push(format!("Undeclared array variable"));
                    }
                }
            }
            self.errors
                .push(format!("Array index must be of the type integer"));
        }
        None
    }

    fn fold_binary_expression(
        &mut self,
        lhs: &Expression,
        rhs: &Expression,
        op: &Token,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        if let Some(typed_lhs) = self.fold_expression(&lhs, st) {
            if let Some(typed_rhs) = self.fold_expression(&rhs, st) {
                if let TypedExpression::Simple(simple_lhs, _addr) = typed_lhs {
                    if let TypedExpression::Simple(simple_rhs, _addr) =
                        typed_rhs
                    {
                        return self.fold_expression_switch(
                            simple_lhs, simple_rhs, op,
                        );
                    } else {
                        self.errors.push(format!(
                            "Right hand side of an operation is of array type"
                        ));
                    }
                } else {
                    self.errors.push(format!(
                        "Left hand side of an operation is of array type"
                    ));
                }
            }
        }
        None
    }

    fn fold_expression_switch(
        &mut self,
        simple_lhs: SimplyTypedExpression,
        simple_rhs: SimplyTypedExpression,
        op: &Token,
    ) -> Option<TypedExpression> {
        match simple_lhs {
            SimplyTypedExpression::Int(e_lhs) => {
                if let SimplyTypedExpression::Int(e_rhs) = simple_rhs {
                    return self.fold_int_expression(e_lhs, e_rhs, op);
                }
            }
            SimplyTypedExpression::String(e_lhs) => {
                if let SimplyTypedExpression::String(e_rhs) = simple_rhs {
                    return self.fold_string_expression(e_lhs, e_rhs, op);
                }
            }
            SimplyTypedExpression::Real(e_lhs) => {
                if let SimplyTypedExpression::Real(e_rhs) = simple_rhs {
                    return self.fold_real_expression(e_lhs, e_rhs, op);
                }
            }
            SimplyTypedExpression::Boolean(e_lhs) => {
                if let SimplyTypedExpression::Boolean(e_rhs) = simple_rhs {
                    return self.fold_boolean_expression(e_lhs, e_rhs, op);
                }
            }
        }
        self.errors.push(format!("Mismatched types"));
        None
    }

    fn fold_int_expression(
        &mut self,
        lhs: TypedExpressionCore,
        rhs: TypedExpressionCore,
        op: &Token,
    ) -> Option<TypedExpression> {
        let typed_lhs =
            TypedExpression::Simple(SimplyTypedExpression::Int(lhs), 0);
        let typed_rhs =
            TypedExpression::Simple(SimplyTypedExpression::Int(rhs), 0);
        let expression = TypedExpressionCore::Binary(
            Box::from(typed_lhs),
            Box::from(typed_rhs),
            op.clone(),
        );
        let maybe_simply_typed = match string_as_opkind(&op.lexeme) {
            OpKind::Addition | OpKind::NumArithmetic | OpKind::Modulo => {
                Some(SimplyTypedExpression::Int(expression))
            }
            OpKind::Relational => {
                Some(SimplyTypedExpression::Boolean(expression))
            }
            _ => None,
        };
        if let Some(ste) = maybe_simply_typed {
            Some(TypedExpression::Simple(ste, 0))
        } else {
            None
        }
    }

    fn fold_boolean_expression(
        &mut self,
        lhs: TypedExpressionCore,
        rhs: TypedExpressionCore,
        op: &Token,
    ) -> Option<TypedExpression> {
        let typed_lhs =
            TypedExpression::Simple(SimplyTypedExpression::Boolean(lhs), 0);
        let typed_rhs =
            TypedExpression::Simple(SimplyTypedExpression::Boolean(rhs), 0);
        let expression = TypedExpressionCore::Binary(
            Box::from(typed_lhs),
            Box::from(typed_rhs),
            op.clone(),
        );
        let maybe_simply_typed = match string_as_opkind(&op.lexeme) {
            OpKind::Relational | OpKind::BoolArithmetic => {
                Some(SimplyTypedExpression::Boolean(expression))
            }
            _ => None,
        };
        if let Some(ste) = maybe_simply_typed {
            Some(TypedExpression::Simple(ste, 0))
        } else {
            None
        }
    }

    fn fold_real_expression(
        &mut self,
        lhs: TypedExpressionCore,
        rhs: TypedExpressionCore,
        op: &Token,
    ) -> Option<TypedExpression> {
        let typed_lhs =
            TypedExpression::Simple(SimplyTypedExpression::Real(lhs), 0);
        let typed_rhs =
            TypedExpression::Simple(SimplyTypedExpression::Real(rhs), 0);
        let expression = TypedExpressionCore::Binary(
            Box::from(typed_lhs),
            Box::from(typed_rhs),
            op.clone(),
        );
        let maybe_simply_typed = match string_as_opkind(&op.lexeme) {
            OpKind::Addition | OpKind::NumArithmetic => {
                Some(SimplyTypedExpression::Real(expression))
            }
            OpKind::Relational => {
                Some(SimplyTypedExpression::Boolean(expression))
            }
            _ => None,
        };
        if let Some(ste) = maybe_simply_typed {
            Some(TypedExpression::Simple(ste, 0))
        } else {
            None
        }
    }

    fn fold_string_expression(
        &mut self,
        lhs: TypedExpressionCore,
        rhs: TypedExpressionCore,
        op: &Token,
    ) -> Option<TypedExpression> {
        let typed_lhs =
            TypedExpression::Simple(SimplyTypedExpression::String(lhs), 0);
        let typed_rhs =
            TypedExpression::Simple(SimplyTypedExpression::String(rhs), 0);
        let expression = TypedExpressionCore::Binary(
            Box::from(typed_lhs),
            Box::from(typed_rhs),
            op.clone(),
        );
        let maybe_simply_typed = match string_as_opkind(&op.lexeme) {
            OpKind::Addition => Some(SimplyTypedExpression::String(expression)),
            OpKind::Relational => {
                Some(SimplyTypedExpression::Boolean(expression))
            }
            _ => None,
        };
        if let Some(ste) = maybe_simply_typed {
            Some(TypedExpression::Simple(ste, 0))
        } else {
            None
        }
    }

    fn fold_unary_expression(
        &mut self,
        op: &Token,
        rhs: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        None
    }
}
