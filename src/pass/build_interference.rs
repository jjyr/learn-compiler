use crate::ast::*;
use crate::graph::Graph;

pub fn build_interference(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    build_interference_inner(
        node_list,
        &info.live_afters,
        &mut info.interference_graph,
        &mut info.move_graph,
    )
}

fn build_interference_inner(
    node_list: Vec<Box<Node>>,
    live_afters: &Vec<LiveSet>,
    interference_graph: &mut Graph<String>,
    move_graph: &mut Graph<String>,
) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    for (node, live_set) in node_list.into_iter().zip(live_afters.iter()) {
        let node = match *node {
            ADDQ { target, arg } => {
                let target_var = target.var_or_reg_name().unwrap();
                for var in live_set {
                    if var != &target_var {
                        interference_graph.insert(var.to_owned(), target_var.clone());
                    }
                }
                ADDQ { target, arg }
            }
            MOVQ { target, source } => {
                let target_var = target.var_or_reg_name().unwrap();
                let source_var_opt = source.var_or_reg_name();
                // record move relation
                if source != target && source_var_opt.is_some() {
                    move_graph.insert(source_var_opt.clone().unwrap(), target_var.clone());
                }
                for var in live_set {
                    if var != &target_var && Some(var) != source_var_opt.as_ref() {
                        interference_graph.insert(var.to_owned(), target_var.to_owned());
                    }
                }
                MOVQ { target, source }
            }
            node @ CALLQ(_) => {
                for var in live_set {
                    interference_graph.insert(var.to_owned(), format!("{:?}", RAX));
                }
                node
            }
            If {
                cond,
                if_exps,
                else_exps,
                if_live_afters,
                else_live_afters,
            } => {
                let if_exps = build_interference_inner(
                    if_exps,
                    &if_live_afters,
                    interference_graph,
                    move_graph,
                );
                let else_exps = build_interference_inner(
                    else_exps,
                    &else_live_afters,
                    interference_graph,
                    move_graph,
                );
                If {
                    cond,
                    if_exps,
                    else_exps,
                    if_live_afters,
                    else_live_afters,
                }
            }
            node => {
                // do nothing
                node
            }
        };

        new_node_list.push(Box::new(node));
    }
    new_node_list
}
