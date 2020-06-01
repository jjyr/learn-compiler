use crate::ast::*;
use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

const WORD: usize = 8;

#[derive(Default)]
struct Status {
    color: Option<usize>,
    conflicts: HashSet<usize>,
}

fn chose_a_color(s: &Status) -> usize {
    for i in 0.. {
        if !s.conflicts.contains(&i) {
            return i;
        }
    }
    panic!("can't chose color")
}

fn find_most_saturated_vertex(
    status: &HashMap<String, Status>,
    interference: &Graph<Node>,
) -> Option<Node> {
    let v = interference
        .iter_vertex()
        .filter(|v| {
            status
                .get(v.var().expect("var"))
                .expect("status")
                .color
                .is_none()
        })
        .max_by_key(|v| interference.get_adjacents_set(v).expect("adjacents").len());
    v.map(Clone::clone)
}

fn color_graph(interference: &mut Graph<Node>) -> HashMap<String, usize> {
    // remove RAX, since we use RAX to patch instructions,
    // so we do not allocate RAX for variables
    // which means RAX wound not be interferenced with other variables / registers
    interference.remove(&Node::RAX);

    // 1. find the most saturated vertex
    // 2. allocate a color
    // 3. mark adjacent vertexes
    let mut status: HashMap<String, Status> = interference
        .iter_vertex()
        .cloned()
        .map(|vertex| (vertex.var().expect("var").to_owned(), Status::default()))
        .collect();
    while let Some(vertex) = find_most_saturated_vertex(&status, interference) {
        let mut s: &mut Status = status.get_mut(vertex.var().expect("var")).expect("vertex");
        let c = chose_a_color(s);
        s.color = Some(c);
        for var in interference.get_adjacents_set(&vertex).expect("adjacents") {
            status
                .get_mut(var.var().expect("var"))
                .unwrap()
                .conflicts
                .insert(c);
        }
    }

    // mapping color to registers
    status
        .into_iter()
        .map(|(var, status)| (var, status.color.expect("allocated")))
        .collect()
}

fn map_var_node(var_to_reg: &HashMap<String, Node>, node: Box<Node>) -> Box<Node> {
    if let Node::Var(var) = node.as_ref() {
        let value = var_to_reg[var].clone();
        Box::new(value)
    } else {
        node
    }
}

pub fn allocate_registers(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    use Node::*;

    let color_map = color_graph(&mut info.interference_graph);
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
        let node = match *node {
            ADDQ { target, arg } => {
                let target = map_var_node(&var_to_reg, target);
                let arg = map_var_node(&var_to_reg, arg);
                Box::new(ADDQ { target, arg })
            }
            MOVQ { target, source } => {
                let target = map_var_node(&var_to_reg, target);
                let source = map_var_node(&var_to_reg, source);
                Box::new(MOVQ { target, source })
            }
            value => Box::new(value),
        };
        new_node_list.push(node);
    }
    info.stack_vars_count = stack_vars_count;
    new_node_list
}
