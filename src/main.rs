mod ast;
mod ast_printer;
mod parser;
mod pass;

use ast_printer::print_ast;
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
}

fn main() {
    test("(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))");
    test("(program (+ 10 2))");
}
