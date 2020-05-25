use crate::ast::{Node, Token, Value};

pub fn partial_eval(node: Box<Node>) -> Box<Node> {
    let Node { token, value } = *node;
    let (token, value) = match value {
        Value::Program(sub_node) => (token, Value::Program(partial_eval(sub_node))),
        Value::Neg(sub_node) => {
            let sub_node = partial_eval(sub_node);
            if let Value::Fixnum(num) = sub_node.value {
                (Token::Fixnum, Value::Fixnum(-num))
            } else {
                (Token::Neg, Value::Neg(sub_node))
            }
        }
        Value::Add(lhs, rhs) => {
            let lhs = partial_eval(lhs);
            let rhs = partial_eval(rhs);
            if lhs.token == Token::Fixnum && rhs.token == Token::Fixnum {
                (
                    Token::Fixnum,
                    Value::Fixnum(lhs.value.fixnum() + rhs.value.fixnum()),
                )
            } else {
                (Token::Add, Value::Add(lhs, rhs))
            }
        }
        _ => (token, value),
    };
    Box::new(Node { token, value })
}
