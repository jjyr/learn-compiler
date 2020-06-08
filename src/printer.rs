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
        Let { name, value, exp } => {
            print!("(let ([{} ", name);
            print_ast(value);
            print!("]) ");
            print_ast(exp);
            print!(")");
        }
        Not(exp) => {
            print!("(not ");
            print_ast(exp);
            print!(")");
        }
        Eq(lhs, rhs) => {
            print!("(== ");
            print_ast(lhs);
            print!(" ");
            print_ast(rhs);
            print!(")");
        }
        b @ True | b @ False => {
            print!("{:?}", b);
        }
        op @ Lt(_, _) | op @ Lte(_, _) | op @ Gt(_, _) | op @ Gte(_, _) => {
            print!("(");
            let (lhs, rhs) = match op {
                Lt(lhs, rhs) => {
                    print!("<");
                    (lhs, rhs)
                }
                Lte(lhs, rhs) => {
                    print!("<=");
                    (lhs, rhs)
                }
                Gt(lhs, rhs) => {
                    print!(">");
                    (lhs, rhs)
                }
                Gte(lhs, rhs) => {
                    print!(">=");
                    (lhs, rhs)
                }
                op => {
                    panic!("unexpected {:?}", op);
                }
            };
            print!(" ");
            print_ast(lhs);
            print!(" ");
            print_ast(rhs);
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
