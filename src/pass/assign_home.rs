use crate::ast::*;
use std::collections::HashMap;

#[derive(Default)]
struct Context {
    var_table: HashMap<String, usize>,
    vars_count: usize,
}

/* assign variables to stack */
fn alloc_stack(cxt: &mut Context, var_name: String) -> Value {
    let offset = cxt.var_table.get(&var_name);
    let offset = match offset {
        Some(n) => *n,
        None => {
            /* assign a stack location to var */
            cxt.vars_count += 1;
            cxt.var_table.insert(var_name, cxt.vars_count);
            cxt.vars_count
        }
    };
    Value::STACK_LOC(-8 * offset as isize)
}

fn assign_home_exp(cxt: &mut Context, node: Box<Node>) -> Box<Node> {
    use Value::*;

    let Node { token, value } = *node;

    match value {
        Add(lhs, rhs) => Box::new(Node::new(
            node.token,
            Add(assign_home_exp(cxt, lhs), assign_home_exp(cxt, rhs)),
        )),
        Var(var_name) => Box::new(Node::new(Token::STACK_LOC, alloc_stack(cxt, var_name))),
        _ => match node.token {
            Token::Read | Token::REG | Token::Fixnum | Token::STACK_LOC => {
                Box::new(Node::new(token, value))
            }
            token => {
                panic!("unexpected {:?}", token);
            }
        },
    }
}

pub fn assign_home(node_list: Vec<Box<Node>>, call_info: &mut CallInfo) -> Vec<Box<Node>> {
    use Value::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    let mut cxt = Context::default();

    for node in node_list {
        let node = match node.value {
            MOVQ {
                target,
                source: arg,
            }
            | ADDQ { target, arg } => {
                let Node {
                    token: target_token,
                    value: target_value,
                } = *target;
                // assign home for target
                let target = match target_value {
                    Var(var_name) => Box::new(Node {
                        token: Token::STACK_LOC,
                        value: alloc_stack(&mut cxt, var_name),
                    }),
                    _ => Box::new(Node {
                        token: target_token,
                        value: target_value,
                    }),
                };
                let arg = assign_home_exp(&mut cxt, arg);
                let value = match node.token {
                    Token::MOVQ => MOVQ {
                        target,
                        source: arg,
                    },
                    Token::ADDQ => ADDQ { target, arg },
                    token => panic!("unexpected {:?}", token),
                };
                Box::new(Node {
                    token: node.token,
                    value,
                })
            }
            value => Box::new(Node::new(node.token, value)),
        };
        new_node_list.push(node);
    }
    // attach call info on program node
    call_info.vars_count = cxt.vars_count;
    new_node_list
}
