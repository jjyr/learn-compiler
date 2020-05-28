use crate::ast::*;
use std::io::{Result, Write};

const WORD: usize = 8;

pub fn print_x86(f: &mut impl Write, node_list: Vec<Box<Node>>, call_info: CallInfo) -> Result<()> {
    use Value::*;

    writeln!(f, ".global main")?;
    writeln!(f, "main:")?;
    writeln!(f, "PUSHQ %%rbp")?;
    writeln!(f, "MOVQ %%rsp, %%rbp")?;
    writeln!(f, "SUBQ ${}, %%rsp", call_info.vars_count * WORD)?;

    for node in node_list {
        let Node { token, value } = *node;
        match value {
            MOVQ { target, source } => {
                writeln!(f, "MOVQ {}, {}", parse_val(target), parse_val(source))?;
            }
            ADDQ { target, arg } => {
                writeln!(f, "ADDQ {}, {}", parse_val(target), parse_val(arg))?;
            }
            CALLQ(symbol) => {
                writeln!(f, "CALLQ {}", symbol)?;
            }
            _ => {
                panic!("unexpected token {:?}", token);
            }
        }
    }

    // print out return value
    writeln!(f, "MOVQ %%rax, %%rdi")?;
    writeln!(f, "CALLQ print_int")?;
    // resume the stack and return 0
    writeln!(f, "ADDQ ${}, %%rsp", call_info.vars_count * WORD)?;
    writeln!(f, "MOVQ $0, %%rax")?;
    writeln!(f, "POPQ %%rbp")?;
    writeln!(f, "retq")?;
    Ok(())
}

fn parse_val(node: Box<Node>) -> String {
    use Value::*;

    let Node { token: _, value } = *node;

    match value {
        Fixnum(n) => format!("{}", n),
        StackLoc(offset) => format!("{}(%%rbp)", offset),
        RAX => "%%rax".to_string(),
        value => {
            panic!("failed to parse node {:?}", value);
        }
    }
}
