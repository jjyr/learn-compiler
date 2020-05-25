#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token {
    Neg,
    Add,
    Fixnum,
    Read,
    Exp,
    Program,
    Let,
    Var,
    Assign,
    REG,
    /* X86 */
    ADDQ,
    MOVQ,
    CALLQ,
    /* registers */
    RAX,
    /* stack location */
    STACK_LOC,
}

#[derive(Debug, Clone)]
pub enum Value {
    Program(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Var(String),
    Let(String, isize, Box<Node>),
    Fixnum(isize),
    Read,
    Assign(String, Box<Node>),
}

impl Value {
    pub fn fixnum(&self) -> isize {
        match self {
            Self::Fixnum(num) => *num,
            _ => panic!("expect Fixnum"),
        }
    }
}

pub struct CallInfo {
    variables_count: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub token: Token,
    pub value: Value,
}

impl Node {
    pub fn new(token: Token, value: Value) -> Self {
        Node { token, value }
    }
}
