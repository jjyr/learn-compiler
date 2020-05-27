use crate::ast::*;

pub fn select_inst(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    use Value::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());

    for node in node_list {
        let Node { token: _, value } = *node;
        match value {
            Assign(var_name, sub_node) => {
                let Node { token, value } = *sub_node;
                let target = Box::new(Node::new(Token::Var, Var(var_name)));
                match value {
                    Add(lhs, rhs) => {
                        let (assign, other) =
                            if lhs.token == Token::Var || lhs.token == Token::Fixnum {
                                (lhs, rhs)
                            } else {
                                (rhs, lhs)
                            };
                        new_node_list.push(Box::new(Node::new(
                            Token::MOVQ,
                            MOVQ {
                                target: target.clone(),
                                source: assign,
                            },
                        )));
                        new_node_list.push(Box::new(Node::new(
                            Token::ADDQ,
                            ADDQ { target, arg: other },
                        )));
                    }

                    Read => {
                        let rax_node = Box::new(Node::new(Token::REG, RAX));
                        let call_node = Box::new(Node::new(Token::CALLQ, CALLQ("read_int")));
                        let move_node = Box::new(Node::new(
                            Token::MOVQ,
                            MOVQ {
                                target,
                                source: rax_node,
                            },
                        ));
                        new_node_list.push(call_node);
                        new_node_list.push(move_node);
                    }

                    value @ Var(_) | value @ Fixnum(_) => {
                        let node = Box::new(Node::new(token, value));
                        let move_node = Box::new(Node::new(
                            Token::MOVQ,
                            MOVQ {
                                target,
                                source: node,
                            },
                        ));
                        new_node_list.push(move_node);
                    }

                    val => {
                        panic!("unexpected {:?}", val);
                    }
                }
            }
            Program(sub_node) => match &sub_node.value {
                Var(_) | Fixnum(_) => {
                    let target = Box::new(Node::new(Token::REG, RAX));
                    new_node_list.push(Box::new(Node::new(
                        Token::MOVQ,
                        MOVQ {
                            target,
                            source: sub_node,
                        },
                    )));
                }
                val => {
                    panic!("unexpected {:?}", val);
                }
            },
            val => {
                panic!("unexpected {:?}", val);
            }
        }
    }

    new_node_list
}
