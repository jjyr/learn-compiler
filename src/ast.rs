#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum Value {
    Var(String),
    Let(String, isize),
    Fixnum(isize),
}

pub struct CallInfo {
    variables_count: usize,
}

#[derive(Debug)]
pub struct Node {
    pub token: Token,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub value: Option<Value>,
}

impl Node {
    pub fn new(token: Token) -> Self {
        Node {
            token,
            lhs: None,
            rhs: None,
            value: None,
        }
    }
}
