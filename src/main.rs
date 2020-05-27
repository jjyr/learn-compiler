mod ast;
mod ast_printer;
mod parser;
mod pass;

use ast_printer::{print_ast, print_stmt};
use parser::Parser;

fn test(s: &str) {
    let mut parser = Parser::new(s.to_string().chars().collect());
    let ast = parser.parse_program();
    println!("inputs:");
    print_ast(ast.clone());
    println!();
    println!("partial eval:");
    let ast = pass::partial_eval(ast);
    print_ast(ast.clone());
    println!();
    println!("uniquify:");
    let ast = pass::uniquify(ast);
    print_ast(ast.clone());
    println!();
    println!("flattern:");
    let ast = pass::flattern(ast);
    print_stmt(ast.clone());
    println!();
    println!("select inst:");
    let ast = pass::select_inst(ast);
    print_stmt(ast.clone());
    println!();
    println!("assign home:");
    let mut call_info = ast::CallInfo::default();
    let ast = pass::assign_home(ast, &mut call_info);
    print_stmt(ast.clone());
    println!();
    println!("patch inst:");
    let ast = pass::patch_inst(ast);
    print_stmt(ast.clone());
    println!();
}

fn main() {
    test("(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))");
    test("(program (+ 10 2))");
}
