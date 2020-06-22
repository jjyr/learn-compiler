use crate::ast::*;
use std::collections::VecDeque;

fn add_var(s: &mut LiveSet, node: &Node) {
    if let Node::Var(var_name) = &node {
        s.insert(var_name.to_owned());
    }
}

fn remove_var(s: &mut LiveSet, node: &Node) {
    if let Node::Var(var_name) = &node {
        s.remove(var_name);
    }
}

/// Check read set and write set of a node
/// return (node, read set, write set)
fn check_read_write(node: Box<Node>, live_set: &mut LiveSet) -> Box<Node> {
    use Node::*;

    let node = match *node {
        MOVZBQ { target, source } | MOVQ { target, source } => {
            remove_var(live_set, &target);
            add_var(live_set, &source);
            MOVQ { target, source }
        }
        ADDQ { target, arg } => {
            // ADDQ write to target var, but also read from it, so we skip it
            add_var(live_set, &target);
            add_var(live_set, &arg);
            ADDQ { target, arg }
        }
        If {
            cond,
            if_exps,
            else_exps,
            ..
        } => {
            // if branch
            let mut if_live_afters: VecDeque<LiveSet> = Default::default();
            let mut if_live_after = live_set.clone();
            let if_exps = uncover_live_inner(if_exps, &mut if_live_after, &mut if_live_afters);

            // else branch
            let mut else_live_afters: VecDeque<LiveSet> = Default::default();
            let mut else_live_after = live_set.clone();
            let else_exps =
                uncover_live_inner(else_exps, &mut else_live_after, &mut else_live_afters);

            // clear live_set
            // use if_live_after | else_live_after | cond as new live_set
            live_set.clear();

            // union if branch
            for n in if_live_after {
                live_set.insert(n);
            }

            // union else branch
            for n in else_live_after {
                live_set.insert(n);
            }

            check_read_write(cond.clone(), live_set);

            If {
                cond,
                if_exps,
                else_exps,
                if_live_afters: if_live_afters.into(),
                else_live_afters: else_live_afters.into(),
            }
        }
        var_node @ Var(_) => {
            // read variable
            add_var(live_set, &var_node);
            var_node
        }
        node => {
            // do nothing
            node
        }
    };
    Box::new(node)
}

fn uncover_live_inner(
    node_list: Vec<Box<Node>>,
    live_after: &mut LiveSet,
    live_afters: &mut VecDeque<LiveSet>,
) -> Vec<Box<Node>> {
    // start from a empty set
    let mut new_node_list = VecDeque::with_capacity(node_list.len());

    // search list reversely
    for node in node_list.into_iter().rev() {
        let node = check_read_write(node, live_after);
        new_node_list.push_front(node);
        live_afters.push_front(live_after.clone());
    }

    assert_eq!(
        new_node_list.len(),
        live_afters.len(),
        "generated live set for each instruction"
    );

    new_node_list.into()
}

pub fn uncover_live(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    let mut initial_live_after: LiveSet = Default::default();
    let mut live_afters: VecDeque<LiveSet> = Default::default();
    let new_node_list = uncover_live_inner(node_list, &mut initial_live_after, &mut live_afters);

    // fix the set of live afters by pending a empty set
    debug_assert!(
        live_afters[0].is_empty(),
        "set before first instruction is empty"
    );
    live_afters.pop_front();
    live_afters.push_back(Default::default());

    info.live_afters = live_afters.into();
    new_node_list
}
