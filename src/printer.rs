use crate::ast::*;

pub fn print_ast(node: Box<Node>) {
    use Node::*;

    match *node {
        Fixnum(num) => print!("{}", num),
        Program(node) => {
            print!("(program ");
            print_ast(node);
            print!(")");
        }
        Neg(node) => {
            print!("(- ");
            print_ast(node);
            print!(")");
        }
        Add(left, right) => {
            print!("(+ ");
            print_ast(left);
            print!(" ");
            print_ast(right);
            print!(")");
        }
        Read => {
            print!("(read)");
        }
        Var(var) => print!("{}", var),
        Let(var, num, node) => {
            print!("(let ([{} {}]) ", var, num);
            print_ast(node);
            print!(")");
        }
        Assign(var, node) => {
            print!("(assign {} ", var);
            print_ast(node);
            print!(")");
        }
        reg @ RAX | reg @ RBX => {
            print!("(reg {:?})", reg);
        }
        MOVQ { target, source } => {
            print!("MOVQ ");
            print_ast(source);
            print!(" ");
            print_ast(target);
        }
        ADDQ { target, arg } => {
            print!("ADDQ ");
            print_ast(arg);
            print!(" ");
            print_ast(target);
        }
        CALLQ(fname) => {
            print!("CALLQ {}", fname);
        }
        StackLoc(offset) => {
            print!("(deref RBP {})", offset);
        }
        val => panic!("unexpected {:?}", val),
    }
}

pub fn print_stmt(node_list: Vec<Box<Node>>) {
    for node in node_list {
        print_ast(node);
        println!();
    }
}

pub fn print_live_set(set_list: &[LiveSet]) {
    for set in set_list {
        println!("{:?}", set);
    }
}
