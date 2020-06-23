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
        reg @ RAX | reg @ RBX | reg @ AL => {
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
        CMPQ(lhs, rhs) => {
            print!("CMPQ ");
            print_ast(lhs);
            print!(" ");
            print_ast(rhs);
        }
        SET(cc, dst) => {
            print!("SET {:?} ", cc);
            print_ast(dst);
        }
        MOVZBQ { source, target } => {
            print!("MOVZBQ ");
            print_ast(source);
            print!(" ");
            print_ast(target);
        }
        JMPIF(cond_code, label) => {
            print!("(jmp-if {:?} {})", cond_code, label);
        }
        JMP(label) => {
            print!("(jmp {})", label);
        }
        Label(label) => {
            print!("(label {})", label);
        }
        StackLoc(offset) => {
            print!("(deref RBP {})", offset);
        }
        If {
            cond,
            if_exps,
            else_exps,
            ..
        } => {
            print!("(if\n (");
            print_ast(cond);
            print!(")\n(\n");
            for node in if_exps {
                print_ast(node);
                println!();
            }
            print!(")\n(\n");
            for node in else_exps {
                print_ast(node);
                println!();
            }
            print!(")\n)");
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
