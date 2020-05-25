use crate::ast::{Node, Token, Value};

pub fn print_ast(node: Box<Node>) {
    use Value::*;
    match node.value {
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
        // STACK_LOC => {
        //     print!("(deref RBP {:?})", node.value);
        // }
        //   REG => {
        //     print!("(reg ");
        //     match (node.value) {
        //     RAX =>
        //       print!("RAX"),
        //     _ =>
        //       panic!("unexpected reg"),
        //     }
        //     print!(")");
        //   }
        //   MOVQ => {
        //     print!("MOVQ ");
        //     print_ast(node.lhs);
        //     print!(" ");
        //     print_ast((ASTNode *)node.value);
        //   }
        //   ADDQ => {
        //     print!("ADDQ ");
        //     print_ast(node.lhs);
        //     print!(" ");
        //     print_ast((ASTNode *)node.value);
        //   }
        //   CALLQ => {
        //     print!("CALLQ %s", (char *)node.value);
        //   }
        _ => {
            panic!("\nprint_ast: failed to parse token {:?}", node.token);
        }
    }
}

pub fn print_stmt(node_list: Vec<Box<Node>>) {
    for node in node_list {
        print_ast(node);
        println!();
    }
}
