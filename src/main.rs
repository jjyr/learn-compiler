mod ast;
mod ast_printer;
mod parser;

use ast_printer::print_ast;
use parser::Parser;

fn main() {
    let source = "(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))";
    let mut parser = Parser::new(source.to_string().chars().collect());
    let ast = parser.parse_program();
    print_ast(ast);
}
