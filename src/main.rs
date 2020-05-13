use std::env;
use std::fs::File;
use std::io::prelude::*;
mod source;
mod scanner;
mod constants;
mod ast;
mod visitor;
mod parser;
mod printvisitor;
mod semanticvisitor;
mod symboltable;
mod codegenvisitor;


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let s = source::read_file(filename);
    let scanner = scanner::build_scanner(s);
    let mut parser = parser::build_parser(scanner);
    let ast = parser.program();
    for e in parser.errors {
        println!("Parsing error: {}", e);
    }
    if let Some(mut uast) = ast {
        let mut pv = printvisitor::PrintVisitor{};
        uast.accept(&mut pv);
        let mut sv = semanticvisitor::SemanticVisitor::new();
        uast.accept(&mut sv);
        let symboltable = sv.get_symbol_table();
        let mut cgv = codegenvisitor::CodeGenVisitor::new(symboltable);
        uast.accept(&mut cgv);
        for e in sv.errors {
            println!("Semantic error: {}", e);
        }
        print!("{}", cgv.get_output());
        let resultname = filename.clone().replace(".minipascal", ".c");
        let mut file = File::create(resultname.as_str())?;
        file.write_all(cgv.get_output().as_bytes())?;
    }
    Ok(())

}
