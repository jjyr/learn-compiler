use crate::ast::*;
use std::collections::VecDeque;

fn add_var(s: &mut LiveSet, node: &Node) {
    if let Value::Var(var_name) = &node.value {
        s.insert(var_name.to_owned());
    }
}

fn remove_var(s: &mut LiveSet, node: &Node) {
    if let Value::Var(var_name) = &node.value {
        s.remove(var_name);
    }
}

/// Check read set and write set of a node
/// return (node, read set, write set)
fn check_read_write(node: Box<Node>, live_set: &mut LiveSet) -> Box<Node> {
    use Value::*;

    match &node.value {
        MOVQ { target, source } => {
            remove_var(live_set, target);
            add_var(live_set, source);
        }
        ADDQ { target, arg } => {
            // ADDQ write to target var, but also read from it, so we skip it
            add_var(live_set, target);
            add_var(live_set, arg);
        }
        _ => {
            // do nothing
        }
    }
    node
}

pub fn uncover_live(node_list: Vec<Box<Node>>, info: &mut CallInfo) -> Vec<Box<Node>> {
    let mut new_node_list: VecDeque<_> = Default::default();
    let mut live_afters: VecDeque<LiveSet> = Default::default();

    // start from a empty set
    let mut live_after: LiveSet = Default::default();

    // search list reversely
    for node in node_list.into_iter().rev() {
        let node = check_read_write(node, &mut live_after);
        new_node_list.push_front(node);
        live_afters.push_front(live_after.clone());
    }

    assert_eq!(
        new_node_list.len(),
        live_afters.len(),
        "generated live set for each instruction"
    );
    info.live_afters = live_afters.into();
    new_node_list.into()
}