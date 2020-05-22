use crate::ast::*;
use crate::scanner::Token;
use crate::typefolder::string_as_opkind;
use crate::typefolder::OpKind;
use crate::visitor::TypedVisitor;
use std::fs::File;
use std::io::prelude::*;

pub struct CodeGenVisitor {
    buffer: String,
    declaration_buffer: String,
    ready_buffer: String,
    free_buffer: String,
    label_no: u32,
}

impl CodeGenVisitor {
    pub fn new() -> CodeGenVisitor {
        return CodeGenVisitor {
            buffer: String::new(),
            declaration_buffer: String::new(),
            ready_buffer: String::new(),
            free_buffer: String::new(),
            label_no: 0,
        };
    }

    fn add_declaration(&mut self, decl: String) {
        self.declaration_buffer.push_str(decl.as_str());
    }

    fn add_code(&mut self, code: String) {
        self.buffer.push_str(code.as_str());
    }

    fn declare(&mut self, address: &Address, var_type: &NodeType) {
        let lhs_text =
            CodeGenVisitor::get_lhs_text_for_item(var_type.clone(), address);
        let text = format!("{};\n", lhs_text);
        self.add_declaration(text);
    }

    fn get_new_label(&mut self) -> String {
        let text = String::from(format!("label{}", self.label_no));
        self.label_no = self.label_no + 1;
        text
    }

    fn type_conversion(source_type: &SimpleType) -> String {
        match source_type {
            SimpleType::Integer | SimpleType::Boolean => String::from("int"),
            SimpleType::Real => String::from("double"),
            SimpleType::String => String::from("char *"),
        }
    }

    fn type_conversion_from_node_type(source_type: &NodeType) -> String {
        match source_type {
            NodeType::Simple(t) => CodeGenVisitor::type_conversion(t),
            NodeType::ArrayOf(t) => {
                format!("{}*", CodeGenVisitor::type_conversion(t))
            }
        }
    }

    fn printf_format_conversion(source_type: NodeType) -> String {
        match source_type {
            NodeType::Simple(t) => match t {
                SimpleType::Integer | SimpleType::Boolean => String::from("%d"),
                SimpleType::Real => String::from("%f"),
                SimpleType::String => String::from("%s"),
            },
            _ => String::from("Error array printing not implemented"),
        }
    }

    pub fn get_output(&self) -> String {
        // TODO: Fix memory management
        self.ready_buffer.clone()
    }

    fn get_lhs_text_for_item(
        source_type: NodeType,
        item_id: &Address,
    ) -> String {
        match source_type {
            NodeType::Simple(t) => {
                if t == SimpleType::String {
                    format!("char *{}", item_id)
                } else {
                    let t = CodeGenVisitor::type_conversion(&t);
                    format!("{} {}", t, item_id)
                }
            }
            NodeType::ArrayOf(t) => {
                format!("{} *{}", CodeGenVisitor::type_conversion(&t), item_id)
            }
        }
    }

    fn relational_operation_converter(operator: String) -> String {
        match operator.as_str() {
            "<>" => String::from("!="),
            "=" => String::from("=="),
            _ => operator,
        }
    }

    fn numeric_expression(
        &mut self,
        lhs_addr: &Address,
        rhs_addr: &Address,
        result_addr: &Address,
        op: &Token,
    ) {
        let target_op =
            CodeGenVisitor::relational_operation_converter(op.lexeme.clone());
        let text = format!(
            "{} = {} {} {};\n",
            result_addr, lhs_addr, target_op, rhs_addr,
        );
        self.buffer.push_str(text.as_str());
    }

    fn string_expression(
        &mut self,
        lhs_addr: &Address,
        rhs_addr: &Address,
        result_addr: &Address,
        op: &Token,
    ) {
        let opkind = string_as_opkind(&op.lexeme);
        let text = match opkind {
            OpKind::Addition => {
                let alloc_text = format!(
                    "{} = (char *) malloc(256);\n",
                    result_addr.clone()
                );
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
            OpKind::Relational => match op.lexeme.as_str() {
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
            self.add_code(t);
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

    fn boolean_expression(
        &mut self,
        lhs_addr: &Address,
        rhs_addr: &Address,
        result_addr: &Address,
        op: &Token,
    ) {
        let src_op = op.lexeme.clone();
        let opkind = string_as_opkind(&src_op);
        let target_op = match opkind {
            OpKind::BoolArithmetic => {
                CodeGenVisitor::boolean_operator_converter(src_op)
            }
            OpKind::Relational => {
                CodeGenVisitor::relational_operation_converter(src_op)
            }
            _ => String::from(""),
        };
        let text = format!(
            "{} = {} {} {};\n",
            result_addr, lhs_addr, target_op, rhs_addr,
        );
        self.add_code(text);
    }

    fn insert_runtime(&mut self, runtime_file: &str) {
        if let Ok(mut file) = File::open(runtime_file.clone()) {
            let mut contents = String::new();
            if let Ok(_r) = file.read_to_string(&mut contents) {
                self.declaration_buffer.push_str(contents.as_str());
            }
        } else {
            println!("Can't open runtime file {}", runtime_file);
        }
    }
    fn visit_literal(
        &mut self,
        token: &Token,
        address: &Address,
        node_type: &NodeType,
    ) {
        self.declare(address, node_type);
        if node_type == &NodeType::Simple(SimpleType::String) {
            self.visit_string_literal(&token.lexeme, address);
        } else {
            let text = format!("{} = {};\n", address, token.lexeme,);
            self.add_code(text);
        }
    }

    fn visit_string_literal(&mut self, literal: &String, address: &Address) {
        let length = &literal.len(); // add one for terminating char
        let line1 = format!(
            "{} = (char*) malloc({} * sizeof(char));\n",
            address, length,
        );
        let line2 = format!("strcpy({}, {});\n", address, literal);
        let line3 = format!("int {}_size = {};\n", address, length);
        self.add_code(line1);
        self.add_code(line2);
        self.add_code(line3);
    }

    fn visit_size(&mut self, array_address: &Address, address: &Address) {
        let text = format!("{} = {}_size;\n", address, array_address);
        self.add_code(text);
    }

    fn visit_variable(&mut self, var: &TypedVariable) {
        match &var.substructure {
            TypedVariableStructure::Simple => (),
            TypedVariableStructure::Indexed(expr) => {
                self.visit_expression(expr)
            }
        }
    }
    fn visit_binary_expression(
        &mut self,
        lhs: &TypedExpression,
        rhs: &TypedExpression,
        op: &Token,
        result_addr: &Address,
        out_type: &NodeType,
    ) {
        self.visit_expression(lhs);
        self.visit_expression(rhs);
        self.declare(&result_addr, out_type);
        match out_type {
            NodeType::ArrayOf(_t) => (), // Arrays not supported on Binary expression
            NodeType::Simple(t) => match t {
                SimpleType::Boolean => self.boolean_expression(
                    &lhs.address,
                    &rhs.address,
                    result_addr,
                    op,
                ),
                SimpleType::Integer | SimpleType::Real => self
                    .numeric_expression(
                        &lhs.address,
                        &rhs.address,
                        result_addr,
                        op,
                    ),
                SimpleType::String => self.string_expression(
                    &lhs.address,
                    &rhs.address,
                    result_addr,
                    op,
                ),
            },
        }
    }

    fn visit_call_expression(
        &mut self,
        address: &Address,
        arguments: &Vec<TypedExpression>,
        out_address: &Address,
    ) {
        for arg in arguments {
            self.visit_expression(arg);
        }
        let args_text = self.args_call_format(arguments);
        let text = format!("{} = {}{};\n", out_address, address, args_text);
        self.add_code(text);
    }

    fn assign_simple(
        &mut self,
        variable: &TypedVariable,
        value: &TypedExpression,
    ) {
        match &variable.substructure {
            TypedVariableStructure::Indexed(index_expr) => {
                self.visit_expression(index_expr);
            }
            _ => (),
        };
        let text = format!("{} = {};\n", &variable.address, &value.address);
        self.add_code(text);
    }

    fn assign_array(
        &mut self,
        variable: &TypedVariable,
        value: &TypedExpression,
        st: SimpleType,
    ) {
        let type_text = CodeGenVisitor::type_conversion(&st);
        let text = format!(
            "memcpy({}, {}, {}_size * sizeof({}));\n",
            variable.address, value.address, variable.address, type_text
        );
        self.add_code(text);
    }

    fn visit_write(&mut self, arguments: &Vec<TypedExpression>) {
        for arg in arguments {
            self.visit_expression(arg);
            let format_text =
                CodeGenVisitor::printf_format_conversion(arg.node_type.clone());
            let text =
                format!("printf(\"{} \", {});", format_text, arg.address);
            self.add_code(text);
        }
        let text = String::from("printf(\"\\n\");\n");
        self.add_code(text);
    }

    fn args_call_format(&mut self, arguments: &Vec<TypedExpression>) -> String {
        let mut text = String::from("(");
        for i in 0..arguments.len() {
            if let Some(arg) = arguments.get(i) {
                text = format!("{} {}", text, arg.address);
            }
            if i < arguments.len() - 1 {
                text = format!("{},", text);
            }
        }
        format!("{})", text)
    }

    fn visit_read(&mut self, variables: &Vec<Box<TypedVariable>>) {
        for var in variables {
            self.visit_variable(var);
        }
        let formats = self.read_scanf_formats(variables);
        let adressess = self.read_address_formats(variables);
        let text = format!("scanf({}, {});\n", formats, adressess);
        self.add_code(text);
    }

    fn read_scanf_formats(
        &mut self,
        variables: &Vec<Box<TypedVariable>>,
    ) -> String {
        let mut text = format!("\"");
        for var in variables {
            let format_text =
                CodeGenVisitor::printf_format_conversion(var.node_type.clone());
            text = format!("{} {}", text, format_text);
        }
        format!("{}\"", text)
    }

    fn read_address_formats(
        &mut self,
        variables: &Vec<Box<TypedVariable>>,
    ) -> String {
        let mut text = String::new();
        for i in 0..variables.len() {
            if let Some(arg) = variables.get(i) {
                text = format!("{} &{}", text, arg.address);
            }
            if i < variables.len() - 1 {
                text = format!("{},", text);
            }
        }
        text
    }

    fn visit_unary(
        &mut self,
        main_node: &TypedExpression,
        rhs: &TypedExpression,
    ) {
        self.visit_expression(rhs);
        self.declare(&main_node.address, &main_node.node_type);
        let text = format!("{} = !{};\n", main_node.address, rhs.address);
        self.add_code(text);
    }

    fn visit_procedure(
        &mut self,
        address: &Address,
        parameters: &Vec<(TypedVariable, TypedTypeDescription)>,
        block: &Vec<TypedStatement>,
    ) {
        let param_string = self.build_param_string(parameters);
        let definition = format!("void {}{} {{\n", address, param_string);
        self.declaration_buffer.push_str(definition.as_str());
        self.visit_block(block);
        self.add_code(format!("return;\n}}\n"));
        self.ready_buffer.push_str(self.declaration_buffer.as_str());
        self.ready_buffer.push_str(self.buffer.as_str());
        self.declaration_buffer = String::new();
        self.buffer = String::new();
    }

    fn build_param_string(&mut self, parameters: &Vec<(TypedVariable, TypedTypeDescription)>) -> String {
        let mut text = String::from("(");
        for i in 0..parameters.len() {
            if let Some(param) = parameters.get(i){
                let (variable, _type_def) = param;
                let type_text = CodeGenVisitor::type_conversion_from_node_type(&variable.node_type);
                text = format!("{}{} {}", text, type_text, variable.address);
            }
            if i < parameters.len() -1 {
                text = format!("{},", text);
            }
        }
        return format!("{})", text);
    }
}

impl TypedVisitor for CodeGenVisitor {
    fn visit_ast(&mut self, node: &TypedAST) {
        let TypedAST::Program(_token, subroutines, main_block) = node;
        self.insert_runtime("src/runtime.c");
        for sub in subroutines {
            self.visit_subroutine(sub);
        }
        self.declaration_buffer.push_str("int main() {\n");
        self.declaration_buffer
            .push_str("int r0 = 0;\nint r1 = 1;\n");
        self.visit_block(main_block);
        self.buffer.push_str("return 0;}\n");
        self.ready_buffer.push_str(self.declaration_buffer.as_str());
        self.ready_buffer.push_str(self.buffer.as_str());
        self.declaration_buffer = String::new();
        self.buffer = String::new();
    }
    fn visit_subroutine(&mut self, node: &TypedSubroutine) {
        match node {
            TypedSubroutine::Function(_t, _p, _b) => (),
            TypedSubroutine::Procedure(address, parameters, block) => {
                self.visit_procedure(address, parameters, block)
            }
        }
    }
    fn visit_block(&mut self, node: &Vec<TypedStatement>) {
        for statement in node {
            self.visit_statement(statement);
        }
    }
    fn visit_statement(&mut self, node: &TypedStatement) {
        match node {
            TypedStatement::Assert(t, expr) => self.visit_assert(t, expr),
            TypedStatement::Assign(var, expr) => self.visit_assign(var, expr),
            TypedStatement::Block(block) => self.visit_block(block),
            TypedStatement::Call(address, args) => self.visit_call(address, args),
            TypedStatement::Declaration(variable, description) => {
                self.visit_declaration(variable, description)
            }
            TypedStatement::If(condition, body, else_body) => {
                self.visit_if(condition, body, else_body)
            }
            TypedStatement::Read(targets) => self.visit_read(targets),
            TypedStatement::Return(token, value) => {
                self.visit_return(token, value)
            }
            TypedStatement::While(condition, body) => {
                self.visit_while(condition, body)
            }
            TypedStatement::Write(args) => self.visit_write(args),
        }
    }

    fn visit_assign(
        &mut self,
        variable: &TypedVariable,
        value: &TypedExpression,
    ) {
        self.visit_expression(value);
        match &variable.node_type {
            NodeType::ArrayOf(st) => {
                self.assign_array(variable, value, st.clone())
            }
            NodeType::Simple(_st) => self.assign_simple(variable, value),
        }
    }

    fn visit_declaration(
        &mut self,
        identifier: &TypedVariable,
        type_description: &TypedTypeDescription,
    ) {
        match type_description {
            TypedTypeDescription::Simple(_token) => {
                self.declare(
                    &identifier.address.clone(),
                    &identifier.node_type,
                );
                println!("declaration of a simple var");
            }
            TypedTypeDescription::Array(_token, size_expr) => {
                self.visit_expression(size_expr);
                println!("wuut");
                match &identifier.node_type {
                    NodeType::ArrayOf(t) => match t {
                        SimpleType::String => (), // special case;,
                        SimpleType::Integer
                        | SimpleType::Real
                        | SimpleType::Boolean => {
                            println!("generating array declaration");
                            self.declare(
                                &identifier.address.clone(),
                                &identifier.node_type,
                            );
                            let type_id = CodeGenVisitor::type_conversion(&t);
                            let size_text = format!(
                                "int {}_size = {};\n",
                                identifier.address, size_expr.address
                            );
                            let alloc_text = format!(
                                "{} = ({}*) malloc({} * sizeof({}));\n",
                                identifier.address,
                                type_id,
                                size_expr.address,
                                type_id
                            );
                            self.add_code(size_text);
                            self.add_code(alloc_text);
                        }
                    },
                    _ => println!("this should not be a simple type"),
                }
            }
        }
    }
    fn visit_if(
        &mut self,
        condition: &TypedExpression,
        body: &TypedStatement,
        else_body: &Option<Box<TypedStatement>>,
    ) {
        let else_label = self.get_new_label();
        let end_label = self.get_new_label();
        self.visit_expression(condition);
        let cond_addr = condition.address.clone();
        let jump_text =
            format!("if ({} != 1 ) {{ goto {}; }}\n", cond_addr, else_label);
        self.add_code(jump_text);
        self.visit_statement(body);
        let jump_over_else = format!("goto {};\n", end_label);
        self.add_code(jump_over_else);
        let else_label_target = format!("{}:\n", else_label);
        self.add_code(else_label_target);
        if let Some(else_body) = else_body {
            self.visit_statement(else_body);
        } else {
            let goto_end = format!("goto {};\n", end_label);
            self.add_code(goto_end);
        }
        let end_label_target = format!("{}:\n", end_label);
        self.add_code(end_label_target);
    }

    fn visit_while(
        &mut self,
        condition: &TypedExpression,
        body: &TypedStatement,
    ) {
        let loop_start_label = self.get_new_label();
        let end_label = self.get_new_label();
        let loop_start_label_target = format!("{}:\n", loop_start_label);
        self.add_code(loop_start_label_target);
        self.visit_expression(condition);
        let cond_addr = condition.address.clone();
        let jump_text =
            format!("if ({} != 1 ) {{ goto {}; }}\n", cond_addr, end_label);
        self.add_code(jump_text);
        self.visit_statement(body);
        let start_jump_text = format!("goto {};", loop_start_label);
        self.add_code(start_jump_text);
        self.add_code(format!("{}:\n", end_label));
    }

    fn visit_assert(&mut self, token: &Token, condition: &TypedExpression) {
        self.visit_expression(condition);
        let cond_addr = condition.address.clone();
        let line = token.row;
        let assert_msg = format!("On line {}\\n", line);
        let text = format!("mp_assert({}, \"{}\");\n", cond_addr, assert_msg);
        self.buffer.push_str(text.as_str());
    }

    fn visit_call(
        &mut self,
        address: &Address,
        arguments: &Vec<TypedExpression>,
    ) {
        println!("Visiting call");
        for arg in arguments {
            self.visit_expression(arg);
        }
        let args_text = self.args_call_format(arguments);
        let text = format!("{}{};\n", address, args_text);
        self.add_code(text);
    }
    fn visit_return(
        &mut self,
        _token: &Token,
        value: &Option<TypedExpression>,
    ) {
        let text = if let Some(return_val) = value {
            self.visit_expression(return_val);
            format!("return {};", return_val.address)
        } else {
            String::from("return;")
        };
        self.add_code(text);
    }
    fn visit_expression(&mut self, node: &TypedExpression) {
        match &node.substructure {
            TypedExpressionStructure::Binary(lhs, rhs) => self
                .visit_binary_expression(
                    &lhs,
                    &rhs,
                    &node.token,
                    &node.address,
                    &node.node_type,
                ),
            TypedExpressionStructure::Call(token, args) => {
                self.visit_call_expression(token, args, &node.address)
            }
            TypedExpressionStructure::Size(array_address) => {
                self.visit_size(&array_address, &node.address)
            }
            TypedExpressionStructure::Unary(expr) => {
                self.visit_unary(node, expr)
            }
            TypedExpressionStructure::Literal => {
                self.visit_literal(&node.token, &node.address, &node.node_type)
            }
            TypedExpressionStructure::Variable(var) => self.visit_variable(var),
        }
    }
}
