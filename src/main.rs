use crate::parser::Parser;
use crate::visitor::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
mod address;
mod opkind;
mod ast;
mod codegenvisitor;
mod constants;
mod parser;
mod printvisitor;
mod scanner;
mod source;
mod symboltable;
mod token;
mod typedast;
mod typefolder;
mod visitor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(file_in) = args.get(1) {
        if let Some(file_out) = args.get(2) {
            match compile(file_in, file_out) {
                Ok(()) => (),
                Err(_field) => print!("Error occurred"),
            }
        } else {
            println!("No output file.");
        }
    } else {
        print!("No input file");
    }
}

fn compile(file_in: &String, file_out: &String) -> std::io::Result<()> { 
    println!("Mini-Pascal compiler by Vili Lipo, Helsinki 2020.\n");
    let maybe_source = source::read_file(file_in);
    if let Ok(s) = maybe_source {
        let scanner = scanner::build_scanner(s);
        let mut parser = Parser::new(scanner);
        let ast = parser.program();
        for e in &parser.errors {
            println!("{}", e);
        }
        if let Some(isast) = ast {
            let mut tf = typefolder::TypeFolder::new();
            let mut table = symboltable::get_symbol_table();
            if let Some(typedast) = tf.fold_ast(&isast, &mut table) {
                for e in tf.get_errors() {
                    println!("{}", e)
                }
                if tf.get_errors().len() == 0 && parser.errors.len() == 0 {
                    let mut cv = codegenvisitor::CodeGenVisitor::new();
                    cv.visit_ast(&typedast);
                    let output = cv.get_output();
                    let mut file = File::create(file_out)?;
                    file.write_all(output.as_bytes())?;
                    println!("Compilation successful.");
                }
            } else {
                for e in tf.get_errors() {
                    println!("{}", e)
                }
            }
        }
    } else if let Err(msg) = maybe_source {
        println!("{}", msg);
    }
    Ok(())
}
