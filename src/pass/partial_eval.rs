use crate::ast::Node;

pub fn partial_eval(node: Box<Node>) -> Box<Node> {
    let node = match *node {
        Node::Program(sub_node) => Node::Program(partial_eval(sub_node)),
        Node::Neg(sub_node) => {
            let sub_node = partial_eval(sub_node);
            if let Node::Fixnum(num) = *sub_node {
                Node::Fixnum(-num)
            } else {
                Node::Neg(sub_node)
            }
        }
        Node::Add(lhs, rhs) => {
            let lhs = partial_eval(lhs);
            let rhs = partial_eval(rhs);
            if lhs.fixnum().is_some() && rhs.fixnum().is_some() {
                Node::Fixnum(lhs.fixnum().unwrap() + rhs.fixnum().unwrap())
            } else {
                Node::Add(lhs, rhs)
            }
        }
        _ => *node,
    };
    Box::new(node)
}
