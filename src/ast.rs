use crate::graph::Graph;
use std::collections::HashSet;
use std::hash::Hash;

/// Token
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Token {
    Neg,
    Add,
    Fixnum,
    Read,
    Program,
    Let,
    Var,
}

/// AST node
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Node {
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

impl Default for Node {
    fn default() -> Self {
        Node::NOP
    }
}

impl Node {
    pub fn var(&self) -> Option<&String> {
        match &self {
            Self::Var(name) => Some(name),
            _ => None,
        }
    }
    pub fn fixnum(&self) -> Option<isize> {
        match self {
            Self::Fixnum(num) => Some(*num),
            _ => None,
        }
    }
}

pub type LiveSet = HashSet<String>;

#[derive(Default)]
pub struct Info {
    pub stack_vars_count: usize,
    pub live_afters: Vec<LiveSet>,
    pub interference_graph: Graph<Node>,
    pub move_graph: Graph<Node>,
}
