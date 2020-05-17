use std::env;
use std::fs::File;
use std::io::prelude::*;
mod ast;
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
        if let Some(typedast) = tf.fold_ast(&isast, &mut table) {
            print!("GOT TYPEDAST");
        } else {
            for e in tf.get_errors() {
                println!("Semantic error: {}", e)
            }
        }
    }
    Ok(())
}
