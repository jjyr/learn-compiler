mod ast;
mod graph;
mod parser;
mod pass;
mod printer;

use parser::Parser;
use printer::{print_ast, print_live_set, print_stmt};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn test_type_check(s: &str) -> Result<ast::Type, String> {
    let mut parser = Parser::new(s.to_string().chars().collect());
    let ast = parser.parse_program();
    println!("inputs:");
    print_ast(ast.clone());
    println!();
    let mut info = ast::Info::default();
    let ret_t = pass::type_check(ast, &mut info);
    match ret_t.as_ref() {
        Ok(ret_t) => println!("type check: OK, ret: {:?}", ret_t),
        Err(msg) => println!("type check: Error, msg: {}", msg),
    }
    println!();
    ret_t
}

fn test_r2(s: &str) {
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
    println!("uncover live:");
    let mut info = ast::Info::default();
    let ast = pass::uncover_live(ast, &mut info);
    print_stmt(ast.clone());
    print_live_set(&info.live_afters);
    println!();
    println!("build interference:");
    let ast = pass::build_interference(ast, &mut info);
    println!("{:?}", info.interference_graph);
}

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
    // println!("assign home:");
    // let mut info = ast::Info::default();
    // let ast = pass::assign_home(ast, &mut info);
    // print_stmt(ast.clone());
    // println!();
    println!("uncover live:");
    let mut info = ast::Info::default();
    let ast = pass::uncover_live(ast, &mut info);
    print_stmt(ast.clone());
    print_live_set(&info.live_afters);
    println!();
    println!("build interference:");
    let ast = pass::build_interference(ast, &mut info);
    println!("{:?}", info.interference_graph);
    println!();
    println!("alloc registers:");
    let ast = pass::allocate_registers(ast, &mut info);
    print_stmt(ast.clone());
    println!();
    println!("patch inst:");
    let ast = pass::patch_inst(ast);
    print_stmt(ast.clone());
    println!();
    println!("print x86:");
    let mut buf = Vec::new();
    pass::print_x86(&mut buf, ast, info).unwrap();
    println!("{}", String::from_utf8(buf.clone()).unwrap());
    println!();
    run_code(buf);
}

fn run_cmd(cmd: String) {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("spawn");
    child.wait().expect("failed to execute process");
}

fn build_runtime() {
    run_cmd("cc -c -o runtime/runtime.o runtime/runtime.c".to_string());
}

fn run_code(source: Vec<u8>) {
    let mut dir = env::current_dir().unwrap();
    dir.push("tmp");
    fs::create_dir(&dir).unwrap();
    let source_file = {
        let mut source_file = dir.clone();
        source_file.push("foo.s");
        source_file.to_str().unwrap().to_string()
    };
    let output_file = {
        let mut output_file = dir.clone();
        output_file.push("foo");
        output_file.to_str().unwrap().to_string()
    };
    {
        let mut f = File::create(&source_file).unwrap();
        f.write(&source).unwrap();
    }
    run_cmd(format!(
        "cc -o {output} runtime/runtime.o {input}",
        input = source_file,
        output = output_file
    ));
    run_cmd(format!("{}", output_file));
    println!();
    fs::remove_dir_all(dir).unwrap();
}

fn main() {
    test_type_check("(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))").unwrap();
    test_type_check("(program (+ 10 2))").unwrap();
    test_type_check("(program (== (+ 10 2) false))").unwrap_err();
    test_type_check("(program (== (not true) false))").unwrap();
    test_type_check("(program (== (> 10 2) false))").unwrap();
    test_type_check("(program (if false 0 42))").unwrap();

    // R2 language
    test_r2("(program (if false 0 42))");
    test_r2("(program (if (== 4 10) 0 42))");

    // R1 language
    build_runtime();
    test("(program (+ (read) (let ([x 32]) (+ (let ([x 10]) x) x))))");
    test("(program (+ 10 2))");
}
