use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token {
    Neg,
    Add,
    Fixnum,
    Read,
    Program,
    Let,
    Var,
    /* Statements */
    Assign,
    REG,
    /* X86 */
    ADDQ,
    MOVQ,
    CALLQ,
    /* stack location */
    StackLoc,
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
    MOVQ {
        target: Box<Node>,
        source: Box<Node>,
    },
    RAX,
    CALLQ(&'static str),
    ADDQ {
        target: Box<Node>,
        arg: Box<Node>,
    },
    StackLoc(isize),
}

impl Value {
    pub fn fixnum(&self) -> isize {
        match self {
            Self::Fixnum(num) => *num,
            _ => panic!("expect Fixnum"),
        }
    }
}

pub type LiveSet = HashSet<String>;

#[derive(Default)]
pub struct CallInfo {
    pub vars_count: usize,
    pub live_afters: Vec<LiveSet>,
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
