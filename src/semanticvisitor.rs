use crate::ast::*;
use crate::symboltable::get_symbol_table;
use crate::symboltable::Entry;
use crate::symboltable::Symboltable;
use crate::visitor::Visitor;

enum OpKind {
    Addition,
    NumArithmetic,
    Modulo,
    Relational,
    BoolArithmetic,
    E,
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

    fn string_as_opkind(op: &String) -> OpKind {
        match op.as_str() {
            "+" => OpKind::Addition,
            "-" | "/" | "*" => OpKind::NumArithmetic,
            "%" => OpKind::Modulo,
            "="| "<>"| "<"| "<="| ">="| ">" => OpKind::Relational,
            "or" | "and"  => OpKind::BoolArithmetic,
            _ => OpKind::E,
        }
    }

    fn numeric_expression(&mut self, node: &mut Expression, type_str: String) {
        let op = node.get_token().lexeme.clone();
        let opkind = SemanticVisitor::string_as_opkind(&op);
        match opkind {
            OpKind::NumArithmetic | OpKind::Addition => {
                node.set_type(type_str.clone())
            },
            OpKind::Modulo => {
                if type_str.as_str() != "integer" {
                    self.errors.push(String::from(
                            "Modulo is applicable only to integers"));
                    node.set_type(String::from("Error"));
                } else {
                    node.set_type(type_str.clone());
                }
            },
            OpKind::Relational => {
                node.set_type(String::from("Boolean"));
            },
            _ => {
                    self.errors.push(format!("Bad OP {} for numeric types", op));
                    node.set_type(String::from("Error"));
            }
        }
    }

    fn string_expression(&mut self, node: &mut Expression) {
        let op = node.get_token().lexeme.clone();
        let opkind = SemanticVisitor::string_as_opkind(&op);
        match opkind {
            OpKind::Addition => {
                node.set_type(String::from("string"));
            },
            OpKind::Relational => {
                node.set_type(String::from("Boolean"));
            },
            _ => {
                self.errors.push(format!("Operator {} does not apply to strings", op));
                node.set_type(String::from("Error"));
            },
        }
    }

    fn bool_expression(&mut self, node: &mut Expression) {
        let op = node.get_token().lexeme.clone();
        let opkind = SemanticVisitor::string_as_opkind(&op);
        match opkind {
            OpKind::Relational => {
                node.set_type(String::from("Boolean"));
            },
            OpKind::BoolArithmetic => {
                node.set_type(String::from("Boolean"));
            },
            _ => {
                self.errors.push(format!("Operator {} does not apply to strings", op));
                node.set_type(String::from("Error"));
            },
        }
    }
}

impl Visitor for SemanticVisitor {
    fn visit(&mut self, node: &mut dyn Node) {
        for child in node.get_children() {
            child.accept(self);
        }
    }
    fn visit_program(&mut self, node: &mut Program) {
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
        if let Some(id_child) = node.get_id_child() {
            if let Some(type_child) = node.get_type_child() {
                let name = id_child.get_token().lexeme.clone();
                let t = type_child.get_token().lexeme.clone();
                if let Some(type_entry) = self.symboltable.lookup(&t) {
                    if let Some(scope) = self.symboltable.current_scope() {
                        let entry_type = type_entry.name.clone();
                        let value = type_entry.value.clone();
                        let addr = self.get_register_id();
                        let entry = Entry {
                            name: name.clone(),
                            category: String::from("variable"),
                            entry_type,
                            value,
                            scope_number: scope.scope_number,
                            address: addr.clone(),
                        };
                        if self.symboltable.in_current_scope(&name) {
                            self.errors.push(String::from("Variable declared twice"));
                        } else {
                            self.symboltable.add_entry(entry);
                            node.set_result_addr(addr);
                        }
                    }
                } else {
                    println!("{}", t);
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
            let var_exists = self.symboltable.is_visible(&lhs.get_token().lexeme);
            if var_exists {
                if let Some(rhs) = node.get_rhs_child() {
                    if lhs.get_type().as_str() != rhs.get_type().as_str() {
                        self.errors
                            .push(String::from("Assignment to uncompatible type."));
                    }
                }
            } else {
                self.errors
                    .push(String::from("Assignment to undeclared variable."));
            }
        }
    }

    // TODO: fix operator type relation
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
                    match tl.as_str() {
                        "integer" | "real" => self.numeric_expression(node, tl),
                        "Boolean" => self.bool_expression(node),
                        "string" => self.string_expression(node),
                        _ => (),
                    }
                }
            }
        }
    }
    fn visit_variable(&mut self, node: &mut Variable) {
        let name = node.get_token().lexeme.clone();
        if let Some(entry) = self.symboltable.lookup(&name) {
            let type_str = entry.entry_type.clone();
            node.set_type(type_str);
            let result_addr = entry.address.clone();
            node.set_result_addr(result_addr);
        }
    }
    fn visit_literal(&mut self, node: &mut Literal) {
        match node.get_token().t_type.as_str() {
            "string_literal" => node.set_type(String::from("string")),
            "real_literal" => node.set_type(String::from("real")),
            "integer_literal" => node.set_type(String::from("integer")),
            _ => println!("{}", node.get_token().t_type),
        }
        let addr = self.get_register_id();
        node.set_result_addr(addr);
    }

    fn visit_call(&mut self, node: &mut Call) {
        println!("in call");
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
            println!("Cond type{}", cond_type);
            if cond_type.as_str() != "Boolean" {
                self.errors
                    .push(String::from("Non Boolean expression in an if statement"));
            }
        }
    }

    fn visit_while(&mut self, node: &mut WhileNode) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(condition) = node.get_condition() {
            let cond_type = condition.get_type();
            println!("Cond type{}", cond_type);
            if cond_type.as_str() != "Boolean" {
                self.errors
                    .push(String::from("Non Boolean expression in an if statement"));
            }
        }
    }
}
