use crate::ast::*;
use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

const WORD: usize = 8;

#[derive(Default)]
struct Status {
    color: Option<usize>,
    conflicts: HashSet<usize>,
}

/// choose a color
fn choose_a_color(
    var: &String,
    status: &HashMap<String, Status>,
    move_relation: &Graph<String>,
) -> usize {
    let node_status = status.get(var).expect("status");

    // pick a color based on move relation
    if let Some(adjacents) = move_relation.get_adjacents_set(var) {
        for related in adjacents {
            if let Some(s) = status.get(related) {
                // use color of related variables if it is possible
                let color = match s.color {
                    Some(c) => c,
                    None => continue,
                };

                if !node_status.conflicts.contains(&color) {
                    return color;
                }
            }
        }
    }

    // pick a color
    for i in 0.. {
        if !node_status.conflicts.contains(&i) {
            return i;
        }
    }

    panic!("can't choose a color")
}

fn find_most_saturated_vertex(
    status: &HashMap<String, Status>,
    interference: &Graph<String>,
) -> Option<String> {
    let v = interference
        .iter_vertex()
        .filter(|v| status.get(*v).expect("status").color.is_none())
        .max_by_key(|v| interference.get_adjacents_set(v).expect("adjacents").len());
    v.map(Clone::clone)
}

fn color_graph(
    interference: &mut Graph<String>,
    move_relation: &mut Graph<String>,
) -> HashMap<String, usize> {
    // remove RAX, since we use RAX to patch instructions,
    // so we do not allocate RAX for variables
    // which means RAX wound not be interferenced with other variables / registers
    interference.remove(&format!("{:?}", Node::RAX));

    // 1. find the most saturated vertex
    // 2. allocate a color
    // 3. mark adjacent vertexes
    let mut status: HashMap<String, Status> = interference
        .iter_vertex()
        .cloned()
        .map(|vertex| (vertex, Status::default()))
        .collect();
    while let Some(vertex) = find_most_saturated_vertex(&status, interference) {
        let c = choose_a_color(&vertex, &status, move_relation);

        // update color
        let mut s: &mut Status = status.get_mut(&vertex).expect("vertex");
        s.color = Some(c);

        // update adjacents' conflicts
        for var in interference.get_adjacents_set(&vertex).expect("adjacents") {
            status.get_mut(var).unwrap().conflicts.insert(c);
        }
    }

    // mapping color to registers
    status
        .into_iter()
        .map(|(var, status)| (var.to_owned(), status.color.expect("allocated")))
        .collect()
}

fn map_var_node(var_to_reg: &HashMap<String, Node>, node: Node) -> Box<Node> {
    if let Node::Var(var) = &node {
        let value = var_to_reg[var].clone();
        Box::new(value)
    } else {
        Box::new(node)
    }
}

fn replace_node(node: Box<Node>, var_to_reg: &HashMap<String, Node>) -> Box<Node> {
    use Node::*;

    match *node {
        ADDQ { target, arg } => {
            let target = map_var_node(var_to_reg, *target);
            let arg = map_var_node(var_to_reg, *arg);
            Box::new(ADDQ { target, arg })
        }
        MOVQ { target, source } => {
            let target = map_var_node(var_to_reg, *target);
            let source = map_var_node(var_to_reg, *source);
            Box::new(MOVQ { target, source })
        }
        MOVZBQ { target, source } => {
            let target = map_var_node(var_to_reg, *target);
            let source = map_var_node(var_to_reg, *source);
            Box::new(MOVZBQ { target, source })
        }
        var_node @ Var(_) => map_var_node(var_to_reg, var_node),
        If {
            cond,
            if_exps,
            else_exps,
            if_live_afters,
            else_live_afters,
        } => {
            let cond = replace_node(cond, var_to_reg);
            let if_exps = if_exps
                .into_iter()
                .map(|node| replace_node(node, var_to_reg))
                .collect();
            let else_exps = else_exps
                .into_iter()
                .map(|node| replace_node(node, var_to_reg))
                .collect();
            Box::new(If {
                cond,
                if_exps,
                else_exps,
                if_live_afters,
                else_live_afters,
            })
        }
        value => Box::new(value),
    }
}

pub fn allocate_registers(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    use Node::*;

    let color_map = color_graph(&mut info.interference_graph, &mut info.move_graph);
    let stack_vars_count = color_map.values().max().cloned().unwrap_or(0);

    // mapping color to registers
    let var_to_reg: HashMap<String, Node> = color_map
        .into_iter()
        .map(|(var, color)| {
            let reg = match color {
                0 => RBX,
                offset => StackLoc(-((offset * WORD) as isize)),
            };
            (var, reg)
        })
        .collect();

    let mut new_node_list = Vec::with_capacity(node_list.len());
    for node in node_list {
        new_node_list.push(replace_node(node, &var_to_reg));
    }
    info.stack_vars_count = stack_vars_count;
    new_node_list
}
