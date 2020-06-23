use crate::ast::*;
use std::io::{Result, Write};

const WORD: usize = 8;

pub fn print_x86(f: &mut impl Write, node_list: Vec<Box<Node>>, info: Info) -> Result<()> {
    use Node::*;

    writeln!(f, ".global main")?;
    writeln!(f, "main:")?;
    writeln!(f, "PUSHQ %rbp")?;
    writeln!(f, "MOVQ %rsp, %rbp")?;
    let mut aligned_stack_vars_count = info.stack_vars_count;
    if info.stack_vars_count % 2 != 0 {
        aligned_stack_vars_count += 1;
    }
    if aligned_stack_vars_count > 0 {
        writeln!(f, "SUBQ ${}, %rsp", aligned_stack_vars_count * WORD)?;
    }

    for node in node_list {
        match *node {
            MOVQ { target, source } => {
                writeln!(f, "MOVQ {}, {}", parse_val(source), parse_val(target))?;
            }
            MOVZBQ { target, source } => {
                writeln!(f, "MOVZBQ {}, {}", parse_val(source), parse_val(target))?;
            }
            ADDQ { target, arg } => {
                writeln!(f, "ADDQ {}, {}", parse_val(arg), parse_val(target))?;
            }
            CALLQ(symbol) => {
                writeln!(f, "CALLQ {}", symbol)?;
            }
            CMPQ(lhs, rhs) => {
                writeln!(f, "CMPQ {}, {}", parse_val(lhs), parse_val(rhs))?;
            }
            SET(cond, reg) => {
                assert_eq!(cond, CondCode::E, "unexpected condition code");
                writeln!(f, "SETE {}", parse_val(reg))?;
            }
            JMPIF(cond, label) => {
                assert_eq!(cond, CondCode::E, "unexpected condition code");
                writeln!(f, "JE {}", label)?;
            }
            JMP(label) => {
                writeln!(f, "JMP {}", label)?;
            }
            Label(label) => {
                writeln!(f, "{}:", label)?;
            }
            _ => {
                panic!("unexpected token {:?}", node);
            }
        }
    }

    // print out return value
    writeln!(f, "MOVQ %rax, %rdi")?;
    writeln!(f, "CALLQ print_int")?;
    // resume the stack and return 0
    writeln!(f, "ADDQ ${}, %rsp", aligned_stack_vars_count * WORD)?;
    writeln!(f, "MOVQ $0, %rax")?;
    writeln!(f, "POPQ %rbp")?;
    writeln!(f, "retq")?;
    Ok(())
}

fn parse_val(node: Box<Node>) -> String {
    use Node::*;

    match *node {
        Fixnum(n) => format!("${}", n),
        StackLoc(offset) => format!("{}(%rbp)", offset),
        RAX => "%rax".to_string(),
        RBX => "%rbx".to_string(),
        AL => "%al".to_string(),
        value => {
            panic!("failed to parse node {:?}", value);
        }
    }
}
