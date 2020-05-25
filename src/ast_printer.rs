use crate::ast::{Node, Token, Value};

pub fn print_ast(node: Box<Node>) {
    use Token::*;
    match node.token {
        Fixnum => match node.value.expect("value") {
            Value::Fixnum(num) => {
                print!("{}", num);
            }
            val => {
                panic!("unexpected {:?}", val);
            }
        },
        Program => {
            print!("(program ");
            print_ast(node.lhs.expect("lhs"));
            print!(")");
        }
        Neg => {
            print!("(- ");
            print_ast(node.lhs.expect("lhs"));
            print!(")");
        }
        Add => {
            print!("(+ ");
            print_ast(node.lhs.expect("lhs"));
            print!(" ");
            print_ast(node.rhs.expect("rhs"));
            print!(")");
        }
        Read => {
            print!("(read)");
        }
        Var => match node.value.expect("value") {
            Value::Var(var) => print!("{}", var),
            val => {
                panic!("unexpected {:?}", val);
            }
        },
        STACK_LOC => {
            print!("(deref RBP {:?})", node.value);
        }
        Let => match node.value.expect("value") {
            Value::Let(var, num) => {
                print!("(let ([{} {}]) ", var, num);
                print_ast(node.lhs.expect("lhs"));
                print!(")");
            }
            val => panic!("unexpected {:?}", val),
        },
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
        //   Assign => {
        //     print!("(assign ");
        //     print_ast((ASTNode *)node.value);
        //     print!(" ");
        //     print_ast(node.lhs);
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

// void print_stmt(ASTNode *p) {
//   while (p != 0) {
//     print_ast(p);
//     print!("\n");
//     p = p.rhs;
//   }
// }
