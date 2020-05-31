use crate::graph::Graph;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Value {
    NOP,
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
    CALLQ(&'static str),
    ADDQ {
        target: Box<Node>,
        arg: Box<Node>,
    },
    StackLoc(isize),
    // registers
    RAX,
    RBX,
}

impl Default for Value {
    fn default() -> Self {
        Value::NOP
    }
}

impl Value {
    pub fn var(&self) -> &String {
        match &self {
            Self::Var(name) => name,
            _ => panic!("expect Var"),
        }
    }
    pub fn fixnum(&self) -> isize {
        match self {
            Self::Fixnum(num) => *num,
            _ => panic!("expect Fixnum"),
        }
    }
}

pub type LiveSet = HashSet<String>;

#[derive(Default)]
pub struct Info {
    pub vars_count: usize,
    pub live_afters: Vec<LiveSet>,
    pub interference_graph: Graph<Value>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    pub token: Token,
    pub value: Value,
}

impl Node {
    pub fn new(token: Token, value: Value) -> Self {
        Node { token, value }
    }
}
