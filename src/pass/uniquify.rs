use crate::ast::Node;
use std::collections::HashMap;

fn rewrite_var(mut name: String, cnt: usize) -> String {
    name.push_str(&format!("_{}", cnt));
    name
}

fn uniquify_inner(node: Box<Node>, cxt: &mut HashMap<String, usize>) -> Box<Node> {
    use Node::*;

    let node = match *node {
        Program(sub_node) => Program(uniquify_inner(sub_node, cxt)),
        Neg(sub_node) => Neg(uniquify_inner(sub_node, cxt)),
        Add(lhs, rhs) => Add(uniquify_inner(lhs, cxt), uniquify_inner(rhs, cxt)),
        Var(var_name) => {
            let count = cxt.get(&var_name).map(|cnt| *cnt).unwrap_or_default();
            let new_var_name = rewrite_var(var_name, count);
            Var(new_var_name)
        }
        Let { name, value, exp } => {
            let count = cxt.get(&name).map(|cnt| *cnt).unwrap_or_default() + 1;
            // increase cnt in sub node
            cxt.insert(name.clone(), count);
            let sub_node = uniquify_inner(exp, cxt);
            // set cnt back
            cxt.insert(name.clone(), count - 1);
            let new_var_name = rewrite_var(name, count);
            Let {
                name: new_var_name,
                value,
                exp: sub_node,
            }
        }
        _ => *node,
    };
    Box::new(node)
}

pub fn uniquify(node: Box<Node>) -> Box<Node> {
    let mut cxt = HashMap::default();
    uniquify_inner(node, &mut cxt)
}
