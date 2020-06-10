use crate::graph::Graph;
use std::collections::{HashMap, HashSet};
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
    True,
    False,
    Not,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    If,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum CondCode {
    E,
    L,
    Le,
    G,
    Ge,
}

/// AST node
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Node {
    NOP,
    Program(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Var(String),
    Let {
        name: String,
        value: Box<Node>,
        exp: Box<Node>,
    },
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
    CMPQ(Box<Node>, Box<Node>),
    SET(CondCode, Box<Node>),
    MOVZBQ {
        source: Box<Node>,
        target: Box<Node>,
    },
    StackLoc(isize),
    // registers
    RAX,
    RBX,
    AL,
    // bool logic
    True,
    False,
    Not(Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),
    Lte(Box<Node>, Box<Node>),
    Gt(Box<Node>, Box<Node>),
    Gte(Box<Node>, Box<Node>),
    If {
        cond: Box<Node>,
        if_exps: Vec<Box<Node>>,
        else_exps: Vec<Box<Node>>,
    },
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

    pub fn boolean(&self) -> Option<bool> {
        match self {
            Self::True => Some(true),
            Self::False => Some(false),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        self.fixnum().is_some() || self.boolean().is_some()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Type {
    Fixnum,
    Boolean,
}

pub type LiveSet = HashSet<String>;

#[derive(Default)]
pub struct Info {
    pub stack_vars_count: usize,
    pub vars_types: HashMap<String, Type>,
    pub live_afters: Vec<LiveSet>,
    pub interference_graph: Graph<Node>,
    pub move_graph: Graph<Node>,
}
