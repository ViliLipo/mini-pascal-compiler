use crate::ast::NodeType;
use crate::ast::*;
use crate::scanner::TokenKind;
use crate::symboltable::get_symbol_table;
use crate::symboltable::ConstructCategory;
use crate::symboltable::Entry;
use crate::symboltable::Symboltable;
use crate::visitor::Visitor;

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

pub struct SemanticVisitor {
    symboltable: Symboltable,
    pub errors: Vec<String>,
    register_number: u32,
}

impl SemanticVisitor {
    pub fn new() -> SemanticVisitor {
        SemanticVisitor {
            symboltable: get_symbol_table(),
            errors: Vec::new(),
            register_number: 0,
        }
    }
    pub fn get_symbol_table(&self) -> Symboltable {
        self.symboltable.clone()
    }

    fn get_register_id(&mut self) -> String {
        let text = String::from(format!("r{}", self.register_number));
        self.register_number = self.register_number + 1;
        text
    }

    fn numeric_expression(&mut self, node: &mut Expression, node_type: NodeType) {
        let op = node.get_token().lexeme.clone();
        let opkind = string_as_opkind(&op);
        match opkind {
            OpKind::NumArithmetic | OpKind::Addition => node.set_type(node_type.clone()),
            OpKind::Modulo => match node_type {
                NodeType::Simple(type_str) => {
                    if type_str.as_str() != "integer" {
                        self.errors
                            .push(String::from("Modulo is applicable only to integers"));
                        node.set_type(NodeType::Unit);
                    } else {
                        node.set_type(NodeType::Simple(String::from("integer")));
                    }
                }
                _ => (),
            },
            OpKind::Relational => {
                node.set_type(NodeType::Simple(String::from("Boolean")));
            }
            _ => {
                self.errors.push(format!("Bad OP {} for numeric types", op));
                node.set_type(NodeType::Simple(String::from("Error")));
            }
        }
    }

    fn string_expression(&mut self, node: &mut Expression) {
        let op = node.get_token().lexeme.clone();
        let opkind = string_as_opkind(&op);
        match opkind {
            OpKind::Addition => {
                node.set_type(NodeType::Simple(String::from("string")));
            }
            OpKind::Relational => {
                node.set_type(NodeType::Simple(String::from("Boolean")));
            }
            _ => {
                self.errors
                    .push(format!("Operator {} does not apply to strings", op));
                node.set_type(NodeType::Unit);
            }
        }
    }

    fn bool_expression(&mut self, node: &mut Expression) {
        let op = node.get_token().lexeme.clone();
        let opkind = string_as_opkind(&op);
        match opkind {
            OpKind::Relational => {
                node.set_type(NodeType::Simple(String::from("Boolean")));
            }
            OpKind::BoolArithmetic => {
                node.set_type(NodeType::Simple(String::from("Boolean")));
            }
            _ => {
                self.errors
                    .push(format!("Operator {} does not apply to strings", op));
                node.set_type(NodeType::Unit);
            }
        }
    }

    fn indexed_variable(&mut self, node: &mut Variable) {
        let name = node.get_token().lexeme.clone();
        if let Some(entry) = self.symboltable.lookup(&name) {
            let result_addr = entry.address.clone();
            if let NodeType::ArrayOf(node_t) = entry.entry_type.clone() {
                if let Some(index_child) = node.get_index_child() {
                    let index_addr = index_child.get_result_addr();
                    match index_child.get_type() {
                        NodeType::Simple(t) => {
                            if t.as_str() != "integer" {
                                self.errors
                                    .push(format!("Array index must be of the type int"));
                            } else {
                                node.set_type(NodeType::Simple(node_t));
                                let addr = format!("{}[{}]", result_addr, index_addr);
                                node.set_result_addr(addr);
                            }
                        }
                        _ => self
                            .errors
                            .push(format!("Array indexes must be of the type of int")),
                    }
                }
            } else {
                node.set_result_addr(result_addr);
                self.errors.push(format!("Can't index a simple type"));
            }
        } else {
            self.errors
                .push(format!("Reference to an undeclared variable {}", name));
        }
    }
}

impl Visitor for SemanticVisitor {
    fn visit(&mut self, node: &mut dyn Node) {
        for child in node.get_children() {
            child.accept(self);
        }
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        for child in node.get_children() {
            child.accept(self);
        }
    }

    fn visit_program(&mut self, node: &mut Program) {
        if let Some(name_child) = node.get_id_child() {
            let prog_id = name_child.get_token().lexeme.clone();
            self.symboltable.add_entry(Entry {
                name: prog_id,
                category: ConstructCategory::Program,
                entry_type: NodeType::Simple(String::from("integer")),
                value: String::from("PROGRAM"),
                scope_number: 0,
                address: String::from(""),
            });
        }
        for child in node.get_children() {
            child.accept(self);
        }
    }
    fn visit_block(&mut self, node: &mut Block) {
        let scope_number = self.symboltable.new_scope_in_current_scope(true);
        node.set_scope_no(scope_number);
        for child in node.get_children() {
            child.accept(self);
        }
        self.symboltable.exit_scope()
    }
    fn visit_declaration(&mut self, node: &mut Declaration) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(id_child) = node.get_id_child() {
            if let Some(type_child) = node.get_type_child() {
                let name = id_child.get_token().lexeme.clone();
                let t = type_child.get_token().lexeme.clone();
                if let Some(type_entry) = self.symboltable.lookup(&t) {
                    let type_name = type_entry.name.clone();
                    let value = type_entry.value.clone();
                    if let Some(scope) = self.symboltable.current_scope() {
                        let addr = self.get_register_id();
                        let entry = if let Some(len_expr) = type_child.get_type_id_len_child() {
                            let entry_type = NodeType::ArrayOf(type_name);
                            Entry {
                                name: name.clone(),
                                category: ConstructCategory::ArrayVar,
                                value: len_expr.get_result_addr(),
                                scope_number: scope.scope_number,
                                entry_type,
                                address: addr.clone(),
                            }
                        } else {
                            let entry_type = NodeType::Simple(type_name);
                            Entry {
                                name: name.clone(),
                                category: ConstructCategory::SimpleVar,
                                entry_type,
                                value,
                                scope_number: scope.scope_number,
                                address: addr.clone(),
                            }
                        };
                        if self.symboltable.in_current_scope(&name) {
                            self.errors.push(String::from("Variable declared twice"));
                        } else {
                            self.symboltable.add_entry(entry);
                            node.set_result_addr(addr);
                        }
                    }
                } else {
                    self.errors
                        .push(String::from("Usage of an undeclared type"));
                }
            }
        }
    }

    fn visit_assignment(&mut self, node: &mut Assignment) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(lhs) = node.get_lhs_child() {
            if let Some(_entry) = self.symboltable.lookup(&lhs.get_token().lexeme) {
                if let Some(rhs) = node.get_rhs_child() {
                    match lhs.get_type() {
                        NodeType::Simple(lt) => match rhs.get_type() {
                            NodeType::Simple(rt) => {
                                if rt != lt {
                                    self.errors.push(format!("Can't assign a {} to {}", rt, lt));
                                }
                            }
                            NodeType::ArrayOf(rt) => self.errors.push(format!(
                                "Can't assign an array of {} to simple type {}",
                                rt, lt
                            )),
                            NodeType::Unit => self.errors.push(format!(
                                "Can't assign a result of an statement to a type {}",
                                lt
                            )),
                        },
                        NodeType::ArrayOf(lt) => {
                            match rhs.get_type() {
                                NodeType::ArrayOf(rt) => {
                                    if rt != lt {
                                        self.errors
                                            .push(format!("Can't assign a {} to {}.", rt, lt));
                                    }
                                } //OK
                                _ => self.errors.push(String::from(
                                    "Can't assign a normal value to array type",
                                )),
                            }
                        }
                        NodeType::Unit => (),
                    }
                }
            } else {
                self.errors
                    .push(String::from("Assignment to undeclared variable."));
            }
        }
    }

    fn visit_expression(&mut self, node: &mut Expression) {
        for child in node.get_children() {
            child.accept(self);
        }
        let result_addr = self.get_register_id();
        node.set_result_addr(result_addr);
        let shared_node = &node;
        if let Some(lhs) = shared_node.get_lhs_child() {
            if let Some(rhs) = shared_node.get_rhs_child() {
                let tl = lhs.get_type();
                let tr = rhs.get_type();
                if tl != tr {
                    self.errors.push(String::from(format!(
                        "Incompatible operands {}, {}",
                        tl, tr
                    )));
                } else {
                    match tl.clone() {
                        NodeType::Simple(s) => match s.as_str() {
                            "integer" | "real" => self.numeric_expression(node, tl),
                            "Boolean" => self.bool_expression(node),
                            "string" => self.string_expression(node),
                            _ => (),
                        },
                        NodeType::ArrayOf(_s) => (),
                        NodeType::Unit => (),
                    }
                }
            }
        }
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        for child in node.get_children() {
            child.accept(self)
        }
        let name = node.get_token().lexeme.clone();
        if node.has_index() {
            self.indexed_variable(node);
        } else {
            if let Some(entry) = self.symboltable.lookup(&name) {
                let result_addr = entry.address.clone();
                node.set_result_addr(result_addr);
                node.set_type(entry.entry_type.clone());
            } else {
                self.errors
                    .push(format!("Reference to an undeclared variable {}", name));
            }
        }
    }

    fn visit_literal(&mut self, node: &mut Literal) {
        match node.get_token().token_kind {
            TokenKind::StringLiteral => node.set_type(NodeType::Simple(String::from("string"))),
            TokenKind::RealLiteral => node.set_type(NodeType::Simple(String::from("real"))),
            TokenKind::IntegerLiteral => node.set_type(NodeType::Simple(String::from("integer"))),
            _ => (),
        }
        let addr = self.get_register_id();
        node.set_result_addr(addr);
    }

    fn visit_call(&mut self, node: &mut Call) {
        for child in node.get_children() {
            child.accept(self);
        }
        let called_id = node.get_token().lexeme.clone();
        let addr = self.get_register_id();
        if let Some(entry) = self.symboltable.lookup(&called_id) {
            match &entry.category {
                ConstructCategory::Function(param_type_list, output_type) => {
                    node.set_type(output_type.clone());
                    node.set_result_addr(addr);
                    if let Some(args) = node.get_arguments() {
                        let children = args.get_children();
                        if children.len() != param_type_list.len() {
                            self.errors
                                .push(format!("Invalid length of an argument list"));
                        } else {
                            for i in 0..param_type_list.len() {
                                let child_type = children[i].get_type();
                                if param_type_list[i] != child_type {
                                    self.errors.push(format!(
                                        "Argument {} is not type {}",
                                        i, param_type_list[i]
                                    ));
                                }
                            }
                        }
                    }
                }
                _ => self.errors.push(format!("Calling a non-callable type.")),
            }
        }
    }

    fn visit_argument(&mut self, node: &mut ArgumentNode) {
        for child in node.get_children() {
            child.accept(self);
        }
    }

    fn visit_if(&mut self, node: &mut IfNode) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(condition) = node.get_condition() {
            let cond_type = condition.get_type();
            if let NodeType::Simple(t) = cond_type {
                if t.as_str() != "Boolean" {
                    self.errors
                        .push(String::from("Non Boolean expression in an if statement"));
                }
            }
        }
    }

    fn visit_while(&mut self, node: &mut WhileNode) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(condition) = node.get_condition() {
            let cond_type = condition.get_type();
            if let NodeType::Simple(t) = cond_type {
                if t.as_str() != "Boolean" {
                    self.errors
                        .push(String::from("Non Boolean expression in a while condition"));
                }
            }
        }
    }
    
    fn visit_assert(&mut self, node: &mut AssertNode) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(condition) = node.get_condition() {
            let cond_type = condition.get_type();
            if let NodeType::Simple(t) = cond_type {
                if t.as_str() != "Boolean" {
                    self.errors
                        .push(String::from("Non Boolean expression in an Assert statement"));
                }
            }
        }
    }
}
