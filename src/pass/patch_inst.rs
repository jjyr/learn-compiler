/// Fix the invalid instructures
/// x86 specific pass
use crate::ast::*;

fn is_patchable(t: &Node) -> bool {
    match t {
        Node::Var(_) | Node::StackLoc(_) => true,
        _ => false,
    }
}

pub fn patch_inst(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());

    for node in node_list {
        match *node {
            MOVQ { target, source } if target == source => {
                // skip unnecessary move
                continue;
            }
            MOVQ { target, source } if is_patchable(&target) && is_patchable(&source) => {
                // patch instruction if the two sides are both StackLoc
                let reg = Box::new(RAX);
                let move_to_reg = Box::new(MOVQ {
                    target: reg.clone(),
                    source,
                });
                new_node_list.push(move_to_reg);

                let node = MOVQ {
                    target,
                    source: reg,
                };
                let patched_inst = Box::new(node);
                new_node_list.push(patched_inst);
            }
            ADDQ { target, arg } if is_patchable(&target) && is_patchable(&arg) => {
                // patch instruction if the two sides are both StackLoc
                let reg = Box::new(RAX);
                let move_to_reg = Box::new(MOVQ {
                    target: reg.clone(),
                    source: arg,
                });
                new_node_list.push(move_to_reg);
                let node = ADDQ {
                    target: reg.clone(),
                    arg: target.clone(),
                };
                let patched_inst = Box::new(node);
                new_node_list.push(patched_inst);
                let move_back = Box::new(MOVQ {
                    target,
                    source: reg,
                });
                new_node_list.push(move_back);
            }

            node => new_node_list.push(Box::new(node)),
        }
    }
    new_node_list
}
