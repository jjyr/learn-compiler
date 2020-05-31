/// Fix the invalid instructures
/// x86 specific pass
use crate::ast::*;

fn is_patchable(t: Token) -> bool {
    use Token::*;

    t == Var || t == StackLoc
}

pub fn patch_inst(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    use Value::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());

    for node in node_list {
        let Node { token, value } = *node;
        match value {
            MOVQ { target, source } if token == Token::MOVQ && target == source => {
                // skip unnecessary move
                continue;
            }
            MOVQ {
                target,
                source: arg,
            }
            | ADDQ { target, arg }
                if is_patchable(target.token) && is_patchable(arg.token) =>
            {
                // patch instruction if the two sides are both StackLoc
                let reg = Box::new(Node::new(Token::REG, RAX));
                let move_to_reg = Box::new(Node::new(
                    Token::MOVQ,
                    MOVQ {
                        target: reg.clone(),
                        source: arg,
                    },
                ));
                new_node_list.push(move_to_reg);
                match token {
                    Token::MOVQ => {
                        let value = MOVQ {
                            target,
                            source: reg,
                        };
                        let patched_inst = Box::new(Node::new(token, value));
                        new_node_list.push(patched_inst);
                    }
                    Token::ADDQ => {
                        let value = ADDQ {
                            target: reg.clone(),
                            arg: target.clone(),
                        };
                        let patched_inst = Box::new(Node::new(token, value));
                        new_node_list.push(patched_inst);
                        let move_back = Box::new(Node::new(
                            Token::MOVQ,
                            MOVQ {
                                target,
                                source: reg,
                            },
                        ));
                        new_node_list.push(move_back);
                    }
                    token => panic!("unexpected token {:?}", token),
                }
            }

            value => new_node_list.push(Box::new(Node::new(token, value))),
        }
    }
    new_node_list
}
