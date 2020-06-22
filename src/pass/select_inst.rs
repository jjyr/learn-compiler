use crate::ast::*;

fn select_one_inst(node: Node, node_list: &mut Vec<Box<Node>>) {
    use Node::*;

    match node {
        Assign(var_name, sub_node) => {
            let target = Box::new(Var(var_name));
            match *sub_node {
                Add(lhs, rhs) => {
                    let (assign, other) = if lhs.var().is_some() || lhs.fixnum().is_some() {
                        (lhs, rhs)
                    } else {
                        (rhs, lhs)
                    };
                    node_list.push(Box::new(MOVQ {
                        target: target.clone(),
                        source: assign,
                    }));
                    node_list.push(Box::new(ADDQ { target, arg: other }));
                }

                Eq(lhs, rhs) => {
                    node_list.push(Box::new(CMPQ(lhs, rhs)));
                    node_list.push(Box::new(SET(CondCode::E, Box::new(AL))));
                    node_list.push(Box::new(MOVZBQ {
                        source: Box::new(AL),
                        target,
                    }));
                }

                Read => {
                    let rax_node = Box::new(RAX);
                    let call_node = Box::new(CALLQ("read_int"));
                    let move_node = Box::new(MOVQ {
                        target,
                        source: rax_node,
                    });
                    node_list.push(call_node);
                    node_list.push(move_node);
                }

                node @ Var(_) | node @ Fixnum(_) => {
                    let node = Box::new(node);
                    let move_node = Box::new(MOVQ {
                        target,
                        source: node,
                    });
                    node_list.push(move_node);
                }

                node @ True | node @ False => {
                    let num = match node {
                        True => 1,
                        False => 0,
                        _ => panic!(),
                    };
                    let node = Box::new(Fixnum(num));
                    let move_node = Box::new(MOVQ {
                        target,
                        source: node,
                    });
                    node_list.push(move_node);
                }

                val => {
                    panic!("unexpected {:?}", val);
                }
            }
        }

        If {
            cond,
            if_exps,
            else_exps,
            if_live_afters,
            else_live_afters,
        } => {
            assert!(cond.var().is_some(), "cond must be var or literal");
            let if_exps = select_inst(if_exps);
            let else_exps = select_inst(else_exps);
            node_list.push(Box::new(If {
                cond,
                if_exps,
                else_exps,
                if_live_afters,
                else_live_afters,
            }));
        }

        Program(sub_node) => match sub_node.as_ref() {
            Var(_) | Fixnum(_) => {
                let target = Box::new(RAX);
                node_list.push(Box::new(MOVQ {
                    target,
                    source: sub_node,
                }));
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

pub fn select_inst(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    let mut new_node_list = Vec::with_capacity(node_list.len());

    for node in node_list {
        select_one_inst(*node, &mut new_node_list);
    }

    new_node_list
}
