use crate::ast::*;
use crate::semanticvisitor::string_as_opkind;
use crate::semanticvisitor::OpKind;
use crate::symboltable::Symboltable;
use crate::visitor::Visitor;
use std::fs::File;
use std::io::prelude::*;

pub struct CodeGenVisitor {
    symboltable: Symboltable,
    buffer: String,
    declaration_buffer: String,
    free_buffer: String,
    label_no: u32,
}

impl CodeGenVisitor {
    pub fn new(symboltable: Symboltable) -> CodeGenVisitor {
        return CodeGenVisitor {
            symboltable,
            buffer: String::new(),
            declaration_buffer: String::new(),
            free_buffer: String::new(),
            label_no: 0,
        };
    }

    fn get_new_label(&mut self) -> String {
        let text = String::from(format!("label{}", self.label_no));
        self.label_no = self.label_no + 1;
        text
    }

    fn type_conversion(source_type: String) -> String {
        match source_type.as_str() {
            "integer" | "Boolean" => String::from("int"),
            "real" => String::from("double"),
            "string" => String::from("char *"),
            _ => String::new(),
        }
    }

    fn type_conversion_from_node_type(source_type: NodeType) -> String {
        match source_type {
            NodeType::Simple(t) => CodeGenVisitor::type_conversion(t),
            NodeType::ArrayOf(t) => format!("{}*", CodeGenVisitor::type_conversion(t)),
            NodeType::Unit => String::from("UNIT TYPE ERROR"),
        }
    }

    fn printf_format_conversion(source_type: NodeType) -> String {
        match source_type {
            NodeType::Simple(t) => match t.as_str() {
                "integer" | "Boolean" => String::from("%d"),
                "real" => String::from("%f"),
                "string" => String::from("%s"),
                _ => String::new(),
            },
            _ => String::from("Error"),
        }
    }

    pub fn get_output(&self) -> String {
        // TODO: Fix memory management
        self.declaration_buffer.clone() + self.buffer.as_str()
    }

    fn get_lhs_text_for_item(source_type: NodeType, item_id: String) -> String {
        match source_type {
            NodeType::Simple(t) => {
                if t == "string" {
                    format!("char *{}", item_id)
                } else {
                    let t = CodeGenVisitor::type_conversion(t);
                    format!("{} {}", t, item_id.clone())
                }
            }
            NodeType::ArrayOf(t) => format!("{} *{}", CodeGenVisitor::type_conversion(t), item_id),
            _ => format!(""),
        }
    }

    fn relational_operation_converter(operator: String) -> String {
        match operator.as_str() {
            "<>" => String::from("!="),
            "=" => String::from("=="),
            _ => operator,
        }
    }
    fn numeric_expression(&mut self, node: &mut Expression, lhs_addr: String, rhs_addr: String) {
        let old_op = node.get_token().lexeme.clone();
        let op = CodeGenVisitor::relational_operation_converter(old_op);
        let text = format!(
            "{} = {} {} {};\n",
            node.get_result_addr(),
            lhs_addr,
            op,
            rhs_addr,
        );
        self.buffer.push_str(text.as_str());
    }

    fn string_expression(&mut self, node: &mut Expression, lhs_addr: String, rhs_addr: String) {
        let op = node.get_token().lexeme;
        let opkind = string_as_opkind(&op);
        let result_addr = node.get_result_addr();
        let text = match opkind {
            OpKind::Addition => {
                let alloc_text = format!("{} = (char *) malloc(256);\n", result_addr.clone());
                let text = alloc_text
                    + format!(
                        "strcpy({},{});\nstrcat({},{});\n",
                        result_addr.clone(),
                        lhs_addr,
                        result_addr,
                        rhs_addr,
                    )
                    .as_str();
                Some(text)
            }
            OpKind::Relational => match op.as_str() {
                "=" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp == 0;\n",
                        lhs_addr, rhs_addr, result_addr,
                    );
                    Some(text)
                }
                "<>" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp != 0;\n",
                        lhs_addr, rhs_addr, result_addr,
                    );
                    Some(text)
                }
                "<" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp < 0;\n",
                        lhs_addr, rhs_addr, result_addr,
                    );
                    Some(text)
                }
                ">" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp < 0;\n",
                        rhs_addr, lhs_addr, result_addr,
                    );
                    Some(text)
                }
                ">=" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp <= 0;\n",
                        rhs_addr, lhs_addr, result_addr,
                    );
                    Some(text)
                }
                "<=" => {
                    let text = format!(
                        "booltmp = strcmp({}, {});\n{} = booltmp <= 0;\n",
                        lhs_addr, rhs_addr, result_addr,
                    );
                    Some(text)
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(t) = text {
            self.buffer.push_str(t.as_str());
        }
    }

    fn boolean_operator_converter(op: String) -> String {
        let t_str = match op.as_str() {
            "or" => "|",
            "and" => "&",
            _ => "",
        };
        String::from(t_str)
    }

    // TODO <> not working
    fn boolean_expression(&mut self, node: &mut Expression, lhs_addr: String, rhs_addr: String) {
        let src_op = node.get_token().lexeme;
        let opkind = string_as_opkind(&src_op);
        let result_addr = node.get_result_addr();
        let target_op = match opkind {
            OpKind::BoolArithmetic => CodeGenVisitor::boolean_operator_converter(src_op),
            OpKind::Relational => CodeGenVisitor::relational_operation_converter(src_op),
            _ => String::from(""),
        };
        let text = format!(
            "{} = {} {} {};\n",
            result_addr, lhs_addr, target_op, rhs_addr,
        );
        self.declaration_buffer.push_str(text.as_str());
    }

    fn insert_runtime(&mut self, runtime_file: String) {
        if let Ok(mut file) = File::open(runtime_file.clone()) {
            let mut contents = String::new();
            if let Ok(_r) = file.read_to_string(&mut contents) {
                self.declaration_buffer.push_str(contents.as_str());
            }
        } else {
            println!("Can't open runtime file {}", runtime_file);
        }
    }
}

impl Visitor for CodeGenVisitor {
    fn visit(&mut self, node: &mut dyn Node) {
        for child in node.get_children() {
            child.accept(self);
        }
    }
    fn visit_program(&mut self, node: &mut Program) {
        self.insert_runtime(String::from("src/runtime.c"));
        self.declaration_buffer.push_str("int main() {\n");
        self.symboltable.enter_scope_with_number(1);
        for child in node.get_children() {
            child.accept(self);
        }
        self.symboltable.exit_scope();
        self.buffer.push_str("return 0;\n}")
    }
    fn visit_block(&mut self, node: &mut Block) {
        let scope_number = node.get_scope_no();
        self.symboltable.enter_scope_with_number(scope_number);
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
            if let Some(entry) = self.symboltable.lookup(&id_child.get_token().lexeme) {
                let node_t = entry.entry_type.clone();
                let target_id = entry.address.clone();
                let lhs_text =
                    CodeGenVisitor::get_lhs_text_for_item(node_t.clone(), target_id.clone());
                let text = format!("{};\n", lhs_text);
                self.declaration_buffer.push_str(text.as_str());
                match node_t.clone() {
                    NodeType::Simple(t) => {
                        if t.as_str() == "string" {
                            let alloc_text = format!("{} = (char *) malloc(256);\n", target_id);
                            self.declaration_buffer.push_str(alloc_text.as_str());
                        }
                    }
                    NodeType::ArrayOf(t) => {
                        if let Some(type_child) = node.get_type_child() {
                            if let Some(size_node) = type_child.get_type_id_len_child() {
                                println!("got size");
                                let type_id =
                                    CodeGenVisitor::type_conversion_from_node_type(node_t.clone());
                                let size_addr = size_node.get_result_addr();
                                let size_text =
                                    format!("int {}_size = {};\n", target_id, size_addr);
                                let alloc_text = format!(
                                    "{} = ({}) malloc({} * sizeof({}));\n",
                                    target_id, type_id, size_addr, type_id
                                );
                                self.buffer.push_str(size_text.as_str());
                                self.buffer.push_str(alloc_text.as_str());
                                if t.as_str() == "string" {
                                    let str_alloc_text =
                                        format!("alloc_str_array({}, {});\n", target_id, size_addr);
                                    let str_free_text =
                                        format!("free_str_array({}, {});\n", target_id, size_addr);
                                    self.buffer.push_str(str_alloc_text.as_str());
                                    self.free_buffer.push_str(str_free_text.as_str());
                                }
                                let free_text = format!("free({});\n", target_id);
                                self.free_buffer.push_str(free_text.as_str());
                            }
                        }
                    }
                    NodeType::Unit => (),
                }
            }
        }
    }

    fn visit_assignment(&mut self, node: &mut Assignment) {
        for child in node.get_children() {
            child.accept(self);
        }
        // TODO: Left hand side Arrays
        if let Some(id_child) = node.get_lhs_child() {
            let target_addr = id_child.get_result_addr();
            if let Some(val_child) = node.get_rhs_child() {
                let value_addr = val_child.get_result_addr();
                match id_child.get_type() {
                    NodeType::Simple(_tl) => match val_child.get_type() {
                        NodeType::Simple(tr) => {
                            if tr.as_str() == "string" {
                                let text = format!("strcpy({}, {});\n", target_addr, value_addr);
                                self.buffer.push_str(text.as_str());
                            } else {
                                let text = format!("{} = {};\n", target_addr, value_addr);
                                self.buffer.push_str(text.as_str());
                            }
                        }
                        _ => (),
                    },
                    NodeType::ArrayOf(_tl) => match val_child.get_type() {
                        NodeType::ArrayOf(tr) => {
                            if let Some(entry) =
                                self.symboltable.lookup(&id_child.get_token().lexeme)
                            {
                                let s_type_id = CodeGenVisitor::type_conversion(tr);
                                let len_addr = entry.value.clone();
                                let text = format!(
                                    "memcpy({}, {}, {} * sizeof({}));\n",
                                    target_addr.clone(),
                                    value_addr,
                                    len_addr,
                                    s_type_id,
                                );
                                self.buffer.push_str(text.as_str());
                            }
                        }
                        _ => (),
                    },
                    NodeType::Unit => (),
                }
            }
        }
    }

    fn visit_expression(&mut self, node: &mut Expression) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(lhs_child) = node.get_lhs_child() {
            if let Some(rhs_child) = node.get_rhs_child() {
                let lhs_addr = lhs_child.get_result_addr();
                let rhs_addr = rhs_child.get_result_addr();
                let result_addr = node.get_result_addr();
                let lhs_type = lhs_child.get_type();
                let type_id = node.get_type();
                let lhs_text = CodeGenVisitor::get_lhs_text_for_item(type_id, result_addr.clone());
                let decl_text = format!("{};\n", lhs_text);
                self.declaration_buffer.push_str(decl_text.as_str());
                match lhs_type {
                    NodeType::Simple(t) => match t.as_str() {
                        "integer" | "real" => {
                            self.numeric_expression(node, lhs_addr, rhs_addr);
                        }
                        "Boolean" => {
                            self.boolean_expression(node, lhs_addr, rhs_addr);
                        }
                        "string" => {
                            self.string_expression(node, lhs_addr, rhs_addr);
                        }
                        _ => (),
                    },
                    NodeType::ArrayOf(_t) => (),
                    NodeType::Unit => (),
                }
            }
        }
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        for child in node.get_children() {
            child.accept(self)
        }
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        for child in node.get_children() {
            child.accept(self)
        }
    }

    fn visit_literal(&mut self, node: &mut Literal) {
        let lit_type = node.get_type();
        if let NodeType::Simple(t) = lit_type.clone() {
            let lit_value = node.get_token().lexeme;
            let addr = node.get_result_addr();
            let lhs_text = CodeGenVisitor::get_lhs_text_for_item(lit_type, addr.clone());
            if t != "string" {
                let text = format!("{} = {};\n", lhs_text, lit_value);
                self.declaration_buffer.push_str(text.as_str());
            } else {
                let text0 = format!("{};\n", lhs_text);
                let text1 = format!(
                    "{} = malloc({});\n",
                    addr,
                    lit_value.clone().chars().count()
                );
                let text2 = format!("strcpy({}, {});\n", addr, lit_value);
                self.declaration_buffer.push_str(text0.as_str());
                self.declaration_buffer.push_str(text1.as_str());
                self.declaration_buffer.push_str(text2.as_str());
            }
        }
    }

    // TODO: Add other things than writeln
    fn visit_call(&mut self, node: &mut Call) {
        for child in node.get_children() {
            child.accept(self);
        }
        match node.get_type() {
            NodeType::Unit => (),
            NodeType::Simple(_type_id) => {
                let lhs_text =
                    CodeGenVisitor::get_lhs_text_for_item(node.get_type(), node.get_result_addr());
                let text = format!("{};\n", lhs_text);
                self.declaration_buffer.push_str(text.as_str());
            }
            NodeType::ArrayOf(_type_id) => (), // TODO: arrays as return values
        };
        match node.get_token().lexeme.as_str() {
            "writeln" => {
                if let Some(args) = node.get_arguments() {
                    for argument in args.get_children() {
                        let addr = argument.get_result_addr().clone();
                        let format_param =
                            CodeGenVisitor::printf_format_conversion(argument.get_type());
                        let text = format!("printf(\"{}\\n\", {});\n", format_param, addr);
                        self.buffer.push_str(text.as_str());
                    }
                }
            }
            "read" => (),
            "size" => {
                let result_addr = node.get_result_addr();
                if let Some(args) = node.get_arguments() {
                    println!("GOT ARGS");
                    if let Some(array) = args.get_children().get(0) {
                        println!("Args got children");
                        if let Some(entry) =
                            self.symboltable.lookup(&array.get_token().lexeme.clone())
                        {
                            let array_addr = entry.address.clone();
                            let text = format!("{} = {}_size;\n", result_addr, array_addr);
                            self.buffer.push_str(text.as_str());
                        }
                    }
                }
            }
            _ => (), // TODO normal functions
        }
    }

    fn visit_argument(&mut self, node: &mut ArgumentNode) {
        for child in node.get_children() {
            child.accept(self);
        }
    }

    fn visit_if(&mut self, node: &mut IfNode) {
        let else_label = self.get_new_label();
        let end_label = self.get_new_label();
        if let Some(condition) = node.get_condition() {
            condition.accept(self);
            let cond_addr = condition.get_result_addr();
            let jump_text = format!("if ({} != 1 ) {{ goto {}; }}\n", cond_addr, else_label);
            self.buffer.push_str(jump_text.as_str());
            if let Some(body) = node.get_body() {
                body.accept(self);
                let jump_over_else = format!("goto {};\n", end_label);
                self.buffer.push_str(jump_over_else.as_str());
            }
            let else_label_target = format!("{}:\n", else_label);
            self.buffer.push_str(else_label_target.as_str());
            if let Some(else_body) = node.get_else_body() {
                else_body.accept(self);
            } else {
                let goto_end = format!("goto {};\n", end_label);
                self.buffer.push_str(goto_end.as_str());
            }
            let end_label_target = format!("{}:\n", end_label);
            self.buffer.push_str(end_label_target.as_str());
        }
    }

    fn visit_while(&mut self, node: &mut WhileNode) {
        let loop_start_label = self.get_new_label();
        let end_label = self.get_new_label();
        let loop_start_label_target = format!("{}:\n", loop_start_label);
        self.buffer.push_str(loop_start_label_target.as_str());
        if let Some(condition) = node.get_condition() {
            condition.accept(self);
            let cond_addr = condition.get_result_addr();
            let jump_text = format!("if ({} != 1 ) {{ goto {}; }}\n", cond_addr, end_label);
            self.buffer.push_str(jump_text.as_str());
            if let Some(body) = node.get_body() {
                body.accept(self);
                let start_jump_text = format!("goto {};", loop_start_label);
                self.buffer.push_str(start_jump_text.as_str());
            }
            self.buffer.push_str(format!("{}:\n", end_label).as_str());
        }
    }

    fn visit_assert(&mut self, node: &mut AssertNode) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(condition) = node.get_condition() {
            let cond_addr = condition.get_result_addr();
            let line = node.get_token().row;
            let assert_msg = format!("On line {}\\n", line);
            let text = format!("mp_assert({}, \"{}\");\n", cond_addr, assert_msg);
            self.buffer.push_str(text.as_str());
        }
    }
}
