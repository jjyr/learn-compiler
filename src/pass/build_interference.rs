use crate::ast::*;

pub fn build_interference(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    for (node, live_set) in node_list.into_iter().zip(info.live_afters.iter()) {
        match &node.as_ref() {
            ADDQ { target, arg: _ } => {
                for var in live_set {
                    let var = Var(var.to_owned());
                    if &var != target.as_ref() {
                        info.interference_graph.insert(var, *target.clone());
                    }
                }
            }
            MOVQ { target, source } => {
                // record move relation
                if source != target {
                    info.move_graph.insert(*source.clone(), *target.clone());
                }
                for var in live_set {
                    let var = Var(var.to_owned());
                    if &var != target.as_ref() && &var != source.as_ref() {
                        info.interference_graph.insert(var, *target.clone());
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
