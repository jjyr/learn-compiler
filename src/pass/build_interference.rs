use crate::ast::*;

pub fn build_interference(node_list: Vec<Box<Node>>, info: &mut Info) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    for (node, live_set) in node_list.into_iter().zip(info.live_afters.iter()) {
        match &node.as_ref() {
            ADDQ { target, arg: _ } => {
                let target_var = target.var_or_reg_name().unwrap();
                for var in live_set {
                    if var != &target_var {
                        info.interference_graph
                            .insert(var.to_owned(), target_var.clone());
                    }
                }
            }
            MOVQ { target, source } => {
                let target_var = target.var_or_reg_name().unwrap();
                let source_var_opt = source.var_or_reg_name();
                // record move relation
                if source != target && source_var_opt.is_some() {
                    info.move_graph
                        .insert(source_var_opt.clone().unwrap(), target_var.clone());
                }
                for var in live_set {
                    if var != &target_var && Some(var) != source_var_opt.as_ref() {
                        info.interference_graph
                            .insert(var.to_owned(), target_var.to_owned());
                    }
                }
            }
            CALLQ(_) => {
                for var in live_set {
                    info.interference_graph
                        .insert(var.to_owned(), format!("{:?}", RAX));
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
