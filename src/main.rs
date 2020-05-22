use crate::visitor::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
mod ast;
mod codegenvisitor;
mod constants;
mod parser;
mod printvisitor;
mod scanner;
mod source;
mod symboltable;
mod typefolder;
mod visitor;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let s = source::read_file(filename);
    let scanner = scanner::build_scanner(s);
    let mut parser = parser::build_parser(scanner);
    let ast = parser.program();
    for e in parser.errors {
        println!("Parsing error: {}", e);
    }
    let mut pv = printvisitor::PrintVisitor::new();
    if let Some(isast) = ast {
        pv.visit_ast(&isast);
        let mut tf = typefolder::TypeFolder::new();
        let mut table = symboltable::get_symbol_table();
        println!("error len{}", tf.get_errors().len());
        if let Some(typedast) = tf.fold_ast(&isast, &mut table) {
            for e in tf.get_errors() {
                println!("Semantic error: {}", e)
            }
            let mut cv = codegenvisitor::CodeGenVisitor::new();
            cv.visit_ast(&typedast);
            let output = cv.get_output();
            println!("{}", output);
            let result_name = filename.clone().replace(".minipascal", ".c");
            let mut file = File::create(result_name.as_str())?;
            file.write_all(output.as_bytes())?;
        }
    }
    Ok(())
}
