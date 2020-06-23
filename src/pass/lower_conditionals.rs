use crate::ast::*;

// TODO delay instruction selection of cond to lower conditionals allows us generate more efficient code
pub fn lower_conditionals(node_list: Vec<Box<Node>>) -> Vec<Box<Node>> {
    use Node::*;

    let mut new_node_list = Vec::with_capacity(node_list.len());
    let mut cnt = 0;
    for node in node_list {
        match *node {
            If {
                cond,
                if_exps,
                else_exps,
                ..
            } => {
                new_node_list.push(Box::new(CMPQ(cond, Box::new(Fixnum(0)))));
                new_node_list.push(Box::new(JMPIF(CondCode::E, format!("then_{}", cnt))));
                new_node_list.extend(else_exps);
                new_node_list.push(Box::new(JMP(format!("end_{}", cnt))));
                new_node_list.push(Box::new(Label(format!("then_{}", cnt))));
                new_node_list.extend(if_exps);
                new_node_list.push(Box::new(Label(format!("end_{}", cnt))));
                cnt += 1;
            }
            node => {
                new_node_list.push(Box::new(node));
            }
        }
    }
    new_node_list
}
