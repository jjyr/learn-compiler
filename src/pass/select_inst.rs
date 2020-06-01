use crate::ast::*;

pub fn select_inst(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());

    for node in node_list {
        match *node {
            Assign(var_name, sub_node) => {
                let target = Box::new(Var(var_name));
                match *sub_node {
                    Add(lhs, rhs) => {
                        let (assign, other) = if lhs.var().is_some() || lhs.fixnum().is_some() {
                            (lhs, rhs)
                        } else {
                            (rhs, lhs)
                        };
                        new_node_list.push(Box::new(MOVQ {
                            target: target.clone(),
                            source: assign,
                        }));
                        new_node_list.push(Box::new(ADDQ { target, arg: other }));
                    }

                    Read => {
                        let rax_node = Box::new(RAX);
                        let call_node = Box::new(CALLQ("read_int"));
                        let move_node = Box::new(MOVQ {
                            target,
                            source: rax_node,
                        });
                        new_node_list.push(call_node);
                        new_node_list.push(move_node);
                    }

                    node @ Var(_) | node @ Fixnum(_) => {
                        let node = Box::new(node);
                        let move_node = Box::new(MOVQ {
                            target,
                            source: node,
                        });
                        new_node_list.push(move_node);
                    }

                    val => {
                        panic!("unexpected {:?}", val);
                    }
                }
            }
            Program(sub_node) => match sub_node.as_ref() {
                Var(_) | Fixnum(_) => {
                    let target = Box::new(RAX);
                    new_node_list.push(Box::new(MOVQ {
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

    new_node_list
}
