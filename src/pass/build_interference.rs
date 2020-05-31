use crate::ast::*;

pub fn build_interference(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    use Value::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    for (node, live_set) in node_list.into_iter().zip(info.live_afters.iter()) {
        match &node.value {
            ADDQ { target, arg: _ } => {
                for var in live_set {
                    let var = Var(var.to_owned());
                    if var != target.value {
                        info.interference_graph.insert(var, target.value.clone());
                    }
                }
            }
            MOVQ { target, source } => {
                for var in live_set {
                    let var = Var(var.to_owned());
                    if var != target.value && var != source.value {
                        info.interference_graph.insert(var, target.value.clone());
                    }
                }
            }
            CALLQ(_) => {
                for var in live_set {
                    let var = Var(var.to_owned());
                    info.interference_graph.insert(var, RAX);
                }
            }
            _ => {
                // do nothing
            }
        }
        new_node_list.push(node);
    }
    new_node_list
}
