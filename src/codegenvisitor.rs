use crate::ast::*;
use crate::semanticvisitor::string_as_opkind;
use crate::semanticvisitor::OpKind;
use crate::symboltable::Entry;
use crate::symboltable::Symboltable;
use crate::visitor::Visitor;

pub struct CodeGenVisitor {
    symboltable: Symboltable,
    buffer: String,
    declaration_buffer: String,
    label_no: u32,
}

impl CodeGenVisitor {
    pub fn new(symboltable: Symboltable) -> CodeGenVisitor {
        return CodeGenVisitor {
            symboltable,
            buffer: String::new(),
            declaration_buffer: String::new(),
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
            "string" => String::from("char"),
            _ => String::new(),
        }
    }

    fn printf_format_conversion(source_type: String) -> String {
        println!("{}", source_type);
        match source_type.as_str() {
            "integer" => String::from("%d"),
            "real" => String::from("%f"),
            "string" => String::from("%s"),
            _ => String::new(),
        }
    }

    pub fn get_output(&self) -> String {
        self.declaration_buffer.clone() + self.buffer.as_str()
    }

    fn get_lhs_text_for_item(source_type: String, item_id: String) -> String {
        if source_type.as_str() == "string" {
            format!("char {}[256]", item_id)
        } else {
            let t = CodeGenVisitor::type_conversion(source_type);
            format!("{} {}", t, item_id.clone())
        }
    }

    fn numeric_operation_converter(operator: String) -> String {
        match operator.as_str() {
            "<>" => String::from("!="),
            _ => operator,
        }
    }
    fn numeric_expression(&mut self, node: &mut Expression, lhs_addr: String, rhs_addr: String) {
        let old_op = node.get_token().lexeme.clone();
        let op = CodeGenVisitor::numeric_operation_converter(old_op);
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
                let text = format!(
                    "strcpy({},{});\nstrcat({},{});\n",
                    result_addr.clone(),
                    lhs_addr,
                    result_addr,
                    rhs_addr,
                );
                Some(text)
            }
            OpKind::Relational => {
                match op.as_str() {
                    "=" => {
                        let text = format!(
                            "booltmp = strcmp({}, {});\n{} = booltmp == 0;\n",
                            lhs_addr,
                            rhs_addr,
                            result_addr,
                            );
                        Some(text)
                    },
                    "<>" => {
                        let text = format!(
                            "booltmp = strcmp({}, {});\n{} = booltmp != 0;\n",
                            lhs_addr,
                            rhs_addr,
                            result_addr,
                            );
                        Some(text)
                    },
                    "<" => {
                        let text = format!(
                            "booltmp = strcmp({}, {});\n{} = booltmp < 0;\n",
                            lhs_addr,
                            rhs_addr,
                            result_addr,
                            );
                        Some(text)
                    },
                    ">" => {
                        let text = format!(
                            "booltmp = strcmp({}, {});\n{} = booltmp < 0;\n",
                            rhs_addr,
                            lhs_addr,
                            result_addr,
                            );
                        Some(text)
                    },
                    _ => None
                }
            }
            _ => None,
        };
        if let Some(t) = text {
            self.buffer.push_str(t.as_str());
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
        self.declaration_buffer.push_str("#include <stdio.h>\n");
        self.declaration_buffer.push_str("#include <string.h>\n");
        self.declaration_buffer.push_str("int booltmp = 0;\n");
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
            let id = id_child.get_token().lexeme;
            if let Some(type_child) = node.get_type_child() {
                let t = type_child.get_token().lexeme.clone();
                let target_id = id_child.get_result_addr();
                let lhs_text = CodeGenVisitor::get_lhs_text_for_item(t, target_id.clone());
                let text = format!("{};\n", lhs_text);
                self.declaration_buffer.push_str(text.as_str());
                if let Some(old_entry) = self.symboltable.lookup(&id) {
                    let mut new_entry: Entry = old_entry.clone();
                    new_entry.address = target_id;
                    self.symboltable.add_entry(new_entry);
                }
            }
        }
    }
    fn visit_assignment(&mut self, node: &mut Assignment) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(id_child) = node.get_lhs_child() {
            let target_addr = id_child.get_result_addr();
            if let Some(val_child) = node.get_rhs_child() {
                let value_addr = val_child.get_result_addr();
                if val_child.get_type().as_str() == "string" {
                    let text = format!("strcpy({}, {});\n", target_addr, value_addr);
                    self.buffer.push_str(text.as_str());
                } else {
                    let text = format!("{} = {};\n", target_addr, value_addr);
                    self.buffer.push_str(text.as_str());
                }
            }
        }
    }
    fn visit_expression(&mut self, node: &mut Expression) {
        for child in node.get_children() {
            child.accept(self);
        }
        let op = node.get_token().lexeme.clone();
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
                match lhs_type.as_str() {
                    "integer" | "real" => {
                        self.numeric_expression(node, lhs_addr, rhs_addr);
                    }
                    "string" => match op.clone().as_str() {
                        "+" => {
                            let text = format!(
                                "strcpy({},{});\nstrcat({},{});\n",
                                result_addr.clone(),
                                lhs_addr,
                                result_addr,
                                rhs_addr,
                            );
                            self.buffer.push_str(text.as_str());
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        let lex = node.get_token().lexeme.clone();
        if let Some(entry) = self.symboltable.lookup(&lex) {
            let addr = entry.address.clone();
            node.set_result_addr(addr);
        }
    }
    fn visit_literal(&mut self, node: &mut Literal) {
        let lit_type = node.get_type();
        let lit_value = node.get_token().lexeme;
        let addr = node.get_result_addr();
        node.set_result_addr(addr.clone());
        let lhs_text = CodeGenVisitor::get_lhs_text_for_item(lit_type, addr);
        let text = format!("{} = {};\n", lhs_text, lit_value);
        self.declaration_buffer.push_str(text.as_str());
    }

    fn visit_call(&mut self, node: &mut Call) {
        for child in node.get_children() {
            child.accept(self);
        }
        if let Some(child) = node.get_children().get(0) {
            let addr = child.get_result_addr().clone();
            let format_param = CodeGenVisitor::printf_format_conversion(child.get_type());
            let text = format!("printf(\"{}\\n\", {});\n", format_param, addr);
            self.buffer.push_str(text.as_str());
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
}
