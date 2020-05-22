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

pub fn string_as_opkind(op: &String) -> OpKind {
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

fn node_type_to_simple_type(node_type: &NodeType) -> SimpleType {
    match node_type {
        NodeType::ArrayOf(t) => t.clone(),
        NodeType::Simple(t) => t.clone(),
    }
}

pub struct TypeFolder {
    errors: Vec<String>,
    address_generator_no: u64,
}

impl TypeFolder {
    pub fn new() -> TypeFolder {
        TypeFolder {
            errors: Vec::new(),
            address_generator_no: 2, // 0 is true, 1 is false so we go from 2
        }
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn get_new_simple_address(&mut self) -> Address {
        self.address_generator_no = self.address_generator_no + 1;
        Address::new_simple(self.address_generator_no)
    }

    fn get_new_indexed_address(
        &mut self,
        address: Address,
        index: Address,
    ) -> Address {
        Address::new_indexed(address, index)
    }

    fn handle_error(&mut self, token: &Token, context: &str) {
        let line = token.row + 1;
        let column = token.column + 1;
        let complete_message = format!(
            "Semantic error: {} On line: {}, column: {}.",
            context, line, column
        );
        self.errors.push(complete_message);
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
        match node {
            Subroutine::Procedure(token, params, body) => {
                self.visit_procedure(token, params, body, st)
            }
            Subroutine::Function(token, params, body) => None,
        }
    } // TODO Functions

    fn visit_procedure(
        &mut self,
        token: &Token,
        params: &Vec<(Token, TypeDescription)>,
        body: &Vec<Statement>,
        st: &mut Symboltable,
    ) -> Option<TypedSubroutine> {
        let upper_scope_no = st.get_current_scope_number();
        let address = self.get_new_simple_address();
        st.new_scope_in_current_scope(false);
        if let Some(typed_params) = self.fold_parameters(params, st) {
            let entry = Entry {
                name: token.lexeme.clone(),
                value: String::new(),
                scope_number: upper_scope_no,
                entry_type: NodeType::Simple(SimpleType::Boolean),
                address: address.clone(),
                category: ConstructCategory::Procedure(
                    self.get_node_type_for_parameters(&typed_params),
                ),
            };
            st.add_entry(entry);
            if let TypedStatement::Block(typed_body) = self.fold_block(body, st)
            {
                st.exit_scope();
                return Some(TypedSubroutine::Procedure(
                    address,
                    typed_params,
                    typed_body,
                ));
            }
        }
        st.exit_scope();
        return None;
    }

    fn get_node_type_for_parameters(
        &mut self,
        parameters: &Vec<(TypedVariable, TypedTypeDescription)>,
    ) -> Vec<NodeType> {
        let mut node_types = Vec::new();
        for param in parameters {
            let (var, _desc) = param;
            node_types.push(var.node_type.clone());
        }
        node_types
    }

    fn fold_parameters(
        &mut self,
        params: &Vec<(Token, TypeDescription)>,
        st: &mut Symboltable,
    ) -> Option<Vec<(TypedVariable, TypedTypeDescription)>> {
        let mut typed_params = Vec::new();
        for param in params {
            let (token, description) = param;
            if let Some(declaration_core) =
                self.fold_typed_declaration_core(token, description, st)
            {
                typed_params.push(declaration_core);
            }
        }
        Some(typed_params)
    }

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
                if &target.node_type == &value.node_type {
                    return Some(TypedStatement::Assign(target, value));
                } else {
                    self.errors.push(format!("Mismatched types in assigment"));
                }
            }
        }
        None
    }

    fn fold_declaration(
        &mut self,
        token: &Token,
        type_description: &TypeDescription,
        st: &mut Symboltable,
    ) -> Option<TypedStatement> {
        let name = token.lexeme.clone();
        if !st.in_current_scope(&name) {
            if let Some(declaration_core) =
                self.fold_typed_declaration_core(token, type_description, st)
            {
                let (typed_var, description) = declaration_core;
                Some(TypedStatement::Declaration(typed_var, description))
            } else {
                None
            }
        } else {
            self.errors.push(format!("Variable declared twice"));
            None
        }
    }

    fn simple_type_from_node_type(
        &mut self,
        node_type: &NodeType,
    ) -> SimpleType {
        match node_type {
            NodeType::Simple(t) => t.clone(),
            NodeType::ArrayOf(t) => t.clone(),
        }
    }

    fn fold_typed_declaration_core(
        &mut self,
        name_token: &Token,
        type_description: &TypeDescription,
        st: &mut Symboltable,
    ) -> Option<(TypedVariable, TypedTypeDescription)> {
        let name = name_token.lexeme.clone();
        if !st.in_current_scope(&name) {
            match type_description {
                TypeDescription::Simple(t) => {
                    self.fold_typed_declaration_simple_core(name_token, t, st)
                }
                TypeDescription::Array(t, e) => {
                    self.fold_typed_declaration_array_core(name_token, t, e, st)
                }
            }
        } else {
            self.errors.push(format!("Variable declared twice"));
            None
        }
    }

    fn fold_typed_declaration_simple_core(
        &mut self,
        name_token: &Token,
        type_token: &Token,
        st: &mut Symboltable,
    ) -> Option<(TypedVariable, TypedTypeDescription)> {
        let type_identifier = type_token.lexeme.clone();
        if let Some(entry_base_type) = type_id_to_simple_type(&type_identifier)
        {
            let entry_type = NodeType::Simple(entry_base_type);
            let name = name_token.lexeme.clone();
            if self.is_valid_type(type_token, st) {
                let address = self.get_new_simple_address();
                let entry = Entry {
                    name,
                    category: ConstructCategory::SimpleVar,
                    scope_number: st.get_current_scope_number(),
                    entry_type: entry_type.clone(),
                    address: address.clone(),
                    value: String::new(),
                };
                st.add_entry(entry);
                Some((
                    TypedVariable {
                        token: name_token.clone(),
                        address,
                        node_type: entry_type.clone(),
                        substructure: TypedVariableStructure::Simple,
                    },
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

    fn fold_typed_declaration_array_core(
        &mut self,
        name_token: &Token,
        type_token: &Token,
        type_expression: &Expression,
        st: &mut Symboltable,
    ) -> Option<(TypedVariable, TypedTypeDescription)> {
        if let Some(typed_expression) =
            self.fold_expression(&type_expression, st)
        {
            if typed_expression.node_type
                == NodeType::Simple(SimpleType::Integer)
            {
                if let Some(type_entry) = st.lookup(&type_token.lexeme) {
                    if type_entry.category == ConstructCategory::TypeId {
                        if st.in_current_scope(&name_token.lexeme) {
                            self.handle_error(
                                name_token,
                                "Variable declared twice",
                            )
                        } else {
                            let address = self.get_new_simple_address();
                            let node_type = NodeType::ArrayOf(
                                self.simple_type_from_node_type(
                                    &type_entry.entry_type,
                                ),
                            );
                            let new_entry = Entry {
                                name: name_token.lexeme.clone(),
                                category: ConstructCategory::ArrayVar,
                                entry_type: node_type.clone(),
                                scope_number: st.get_current_scope_number(),
                                value: String::new(),
                                address: address.clone(),
                            };
                            st.add_entry(new_entry);
                            let type_description = TypedTypeDescription::Array(
                                type_token.clone(),
                                typed_expression,
                            );
                            return Some((
                                TypedVariable {
                                    node_type: node_type.clone(),
                                    token: name_token.clone(),
                                    address,
                                    substructure:
                                        TypedVariableStructure::Simple,
                                },
                                type_description,
                            ));
                        }
                    } else {
                        let msg =
                            format!("{} is not a type", type_token.lexeme);
                        self.handle_error(type_token, msg.as_str());
                    }
                } else {
                    self.handle_error(
                        type_token,
                        "Usage of an undeclared type",
                    );
                }
            } else {
                self.handle_error(
                    &typed_expression.token,
                    "Array size must be of type integer",
                );
            }
        }
        None
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
                return match &typed_condition.node_type {
                    NodeType::Simple(simple_type) => match simple_type {
                        SimpleType::Boolean => {
                            if let Some(eb) = else_body {
                                if let Some(typed_eb) =
                                    self.fold_statement(eb, st)
                                {
                                    Some(TypedStatement::If(
                                        typed_condition,
                                        Box::from(typed_body),
                                        Some(Box::from(typed_eb)),
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                Some(TypedStatement::If(
                                    typed_condition,
                                    Box::from(typed_body),
                                    None,
                                ))
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                };
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
                if typed_condition.node_type
                    == NodeType::Simple(SimpleType::Boolean)
                {
                    return Some(TypedStatement::While(
                        typed_condition,
                        Box::from(typed_body),
                    ));
                } else {
                    self.handle_error(
                        &typed_condition.token,
                        "While condition must be of Boolean type",
                    );
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
        if let Some(typed_condition) = self.fold_expression(condition, st) {
            if typed_condition.node_type
                == NodeType::Simple(SimpleType::Boolean)
            {
                return Some(TypedStatement::Assert(
                    token.clone(),
                    typed_condition,
                ));
            } else {
                self.handle_error(
                    token,
                    "Assert condition must be of Boolean type",
                );
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
                        token, &entry, parameters, out_type, typedargs,
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
        let ok_special = match entry.name.as_str() {
            "size" => {
                if let Some(argument) = arguments.get(0) {
                    if let NodeType::ArrayOf(_t) = &argument.node_type {
                        Some(argument)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(arg) = ok_special {
            Some(TypedExpression {
                token: token.clone(),
                address: self.get_new_simple_address(),
                node_type: NodeType::Simple(SimpleType::Integer),
                substructure: TypedExpressionStructure::Size(
                    arg.address.clone(),
                ),
            })
        } else {
            None
        }
    }

    fn fold_regular_call_expression(
        &mut self,
        token: &Token,
        entry: &Entry,
        parameters: &Vec<NodeType>,
        out_type: &NodeType,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedExpression> {
        if self.match_params_to_arguments(parameters, &arguments) {
            Some(TypedExpression {
                token: token.clone(),
                address: self.get_new_simple_address(),
                node_type: out_type.clone(),
                substructure: TypedExpressionStructure::Call(
                    entry.address.clone(),
                    arguments,
                ),
            })
        } else {
            None
        }
    }

    fn match_params_to_arguments(
        &mut self,
        parameters: &Vec<NodeType>,
        arguments: &Vec<TypedExpression>,
    ) -> bool {
        let mut arg_types = Vec::new();
        for arg in arguments {
            arg_types.push(arg.node_type.clone());
        }
        for i in 0..parameters.len() {
            if let Some(param) = parameters.get(i) {
                if let Some(arg) = arg_types.get(i) {
                    if param != arg {
                        self.errors.push(format!(
                            "Argument type does not match parameters"
                        ));
                        return false;
                    }
                } else {
                    self.errors.push(format!(
                        "Arguments lenght does not match parameters"
                    ));
                    return false;
                }
            }
        }
        true
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
                self.handle_error(&token, "Failed to check argument");
            }
        }
        if let Some(entry) = st.lookup(&token.lexeme) {
            match &entry.category {
                ConstructCategory::Special => {
                    self.fold_special_call_stmnt(token, entry, typedargs)
                }
                ConstructCategory::Procedure(parameters) => {
                    self.fold_regular_call(token, entry, parameters, typedargs)
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
            "read" => Some(TypedStatement::Read(self.read_args(arguments))),
            "writeln" => Some(TypedStatement::Write(arguments)),
            _ => None,
        }
    }

    fn read_args(
        &mut self,
        arguments: Vec<TypedExpression>,
    ) -> Vec<Box<TypedVariable>> {
        let mut vars = Vec::new();
        for arg in arguments {
            match arg.substructure {
                TypedExpressionStructure::Variable(typed_var) => {
                    vars.push(typed_var);
                }
                _ => self.errors.push(format!("Non variable argument in read")),
            }
        }
        vars
    }

    fn fold_regular_call(
        &mut self,
        token: &Token,
        entry: &Entry,
        parameters: &Vec<NodeType>,
        arguments: Vec<TypedExpression>,
    ) -> Option<TypedStatement> {
        if self.match_params_to_arguments(parameters, &arguments) {
            Some(TypedStatement::Call(entry.address.clone(), arguments))
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
        let maybe_node_type = match token.token_kind {
            TokenKind::IntegerLiteral => Some(SimpleType::Integer),
            TokenKind::StringLiteral => Some(SimpleType::String),
            TokenKind::RealLiteral => Some(SimpleType::Real),
            _ => {
                self.errors.push(format!("Non literal parsed as one"));
                None
            }
        };
        if let Some(node_type) = maybe_node_type {
            Some(TypedExpression {
                token: token.clone(),
                address: self.get_new_simple_address(),
                node_type: NodeType::Simple(node_type),
                substructure: TypedExpressionStructure::Literal,
            })
        } else {
            None
        }
    }

    fn fold_variable_as_expression(
        &mut self,
        var: &Variable,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        if let Some(variable) = self.fold_variable(var, st) {
            Some(TypedExpression {
                token: variable.token.clone(),
                address: variable.address.clone(),
                node_type: variable.node_type.clone(),
                substructure: TypedExpressionStructure::Variable(Box::from(
                    variable,
                )),
            })
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
            Some(TypedVariable {
                token: token.clone(),
                address: entry.address.clone(),
                node_type: entry.entry_type.clone(),
                substructure: TypedVariableStructure::Simple,
            })
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
            if let Some(entry) = st.lookup(&token.lexeme) {
                let node_type = NodeType::Simple(node_type_to_simple_type(
                    &entry.entry_type,
                ));
                match &typed_expression.node_type.clone() {
                    NodeType::Simple(simple_type) => match simple_type {
                        SimpleType::Integer => Some(TypedVariable {
                            token: token.clone(),
                            address: self.get_new_indexed_address(
                                entry.address.clone(),
                                typed_expression.address.clone(),
                            ),
                            node_type,
                            substructure: TypedVariableStructure::Indexed(
                                typed_expression,
                            ),
                        }),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
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
                if &typed_lhs.node_type == &typed_rhs.node_type {
                    let maybe_node_type = match &typed_lhs.node_type {
                        NodeType::ArrayOf(_st) => None,
                        NodeType::Simple(simple_type) => match simple_type {
                            SimpleType::Integer => {
                                self.type_integer_expression(op)
                            }
                            SimpleType::Boolean => {
                                self.type_boolean_expression(op)
                            }
                            SimpleType::String => {
                                self.type_string_expression(op)
                            }
                            SimpleType::Real => self.type_real_expression(op),
                        },
                    };
                    if let Some(node_type) = maybe_node_type {
                        let substructure = TypedExpressionStructure::Binary(
                            Box::from(typed_lhs),
                            Box::from(typed_rhs),
                        );
                        return Some(TypedExpression {
                            address: self.get_new_simple_address(),
                            token: op.clone(),
                            node_type,
                            substructure,
                        });
                    }
                }
            }
        }
        None
    }

    fn type_integer_expression(&mut self, op: &Token) -> Option<NodeType> {
        match string_as_opkind(&op.lexeme) {
            OpKind::Addition | OpKind::NumArithmetic | OpKind::Modulo => {
                Some(NodeType::Simple(SimpleType::Integer))
            }
            OpKind::Relational => Some(NodeType::Simple(SimpleType::Boolean)),
            _ => {
                self.errors.push(format!("Bad operator for integer"));
                None
            }
        }
    }

    fn type_string_expression(&mut self, op: &Token) -> Option<NodeType> {
        match string_as_opkind(&op.lexeme) {
            OpKind::Addition => Some(NodeType::Simple(SimpleType::String)),
            OpKind::Relational => Some(NodeType::Simple(SimpleType::Boolean)),
            _ => {
                self.errors.push(format!("Bad operator for string"));
                None
            }
        }
    }

    fn type_real_expression(&mut self, op: &Token) -> Option<NodeType> {
        match string_as_opkind(&op.lexeme) {
            OpKind::Addition | OpKind::NumArithmetic => {
                Some(NodeType::Simple(SimpleType::Real))
            }
            OpKind::Relational => Some(NodeType::Simple(SimpleType::Boolean)),
            _ => {
                self.errors.push(format!("Bad operator for integer"));
                None
            }
        }
    }

    fn type_boolean_expression(&mut self, op: &Token) -> Option<NodeType> {
        match string_as_opkind(&op.lexeme) {
            OpKind::Relational | OpKind::BoolArithmetic => {
                Some(NodeType::Simple(SimpleType::Boolean))
            }
            _ => {
                self.errors.push(format!("Bad operator for integer"));
                None
            }
        }
    }

    fn fold_unary_expression(
        &mut self,
        op: &Token,
        rhs: &Expression,
        st: &mut Symboltable,
    ) -> Option<TypedExpression> {
        match op.token_kind {
            TokenKind::Not => {
                if let Some(rhs_typed) = self.fold_expression(rhs, st) {
                    if rhs_typed.node_type
                        == NodeType::Simple(SimpleType::Boolean)
                    {
                        Some(TypedExpression {
                            node_type: NodeType::Simple(SimpleType::Boolean),
                            address: self.get_new_simple_address(),
                            token: op.clone(),
                            substructure: TypedExpressionStructure::Unary(
                                Box::from(rhs_typed),
                            ),
                        })
                    } else {
                        self.errors
                            .push(format!("not-operator is for Booleans only"));
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
