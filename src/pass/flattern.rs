use crate::ast::*;

pub fn flattern(node: Box<Node>) -> Vec<Box<Node>> {
    let mut node_list = Vec::new();
    let mut context = Context::default();
    context.flattern_inner(node, &mut node_list);
    node_list
}

#[derive(Default)]
struct VarAlloc(usize);

impl VarAlloc {
    fn alloc(&mut self) -> String {
        let var_name = format!("tmp.{}", self.0);
        self.0 += 1;
        var_name
    }
}

#[derive(Default)]
struct Context {
    var_allocator: VarAlloc,
}

impl Context {
    fn flattern_inner(&mut self, node: Box<Node>, node_list: &mut Vec<Box<Node>>) -> Box<Node> {
        use Node::*;

        match *node {
            Program(sub_node) => {
                let sub_node = self.flattern_inner(sub_node, node_list);
                let node = Box::new(Program(sub_node.clone()));
                node_list.push(node);
                sub_node
            }
            Neg(sub_node) => {
                let sub_node = self.flattern_inner(sub_node, node_list);
                let neg_node = Box::new(Neg(sub_node));
                let var_name = self.var_allocator.alloc();
                let assign_node = Box::new(Assign(var_name.clone(), neg_node));
                node_list.push(assign_node);
                return Box::new(Var(var_name));
            }
            Read => {
                let node = Box::new(Read);
                let var_name = self.var_allocator.alloc();
                let assign_node = Box::new(Assign(var_name.clone(), node));
                node_list.push(assign_node);
                return Box::new(Var(var_name));
            }
            v @ Add(..) | v @ Eq(..) | v @ Gt(..) | v @ Gte(..) | v @ Lt(..) | v @ Lte(..) => {
                let node = match v {
                    Add(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Add(lhs_var, rhs_var))
                    }
                    Eq(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Eq(lhs_var, rhs_var))
                    }
                    Gt(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Gt(lhs_var, rhs_var))
                    }
                    Gte(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Gte(lhs_var, rhs_var))
                    }
                    Lt(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Lt(lhs_var, rhs_var))
                    }
                    Lte(lhs, rhs) => {
                        let lhs_var = self.flattern_inner(lhs, node_list);
                        let rhs_var = self.flattern_inner(rhs, node_list);
                        Box::new(Lte(lhs_var, rhs_var))
                    }
                    v => panic!("unexpected {:?}", v),
                };
                let var_name = self.var_allocator.alloc();
                let assign_node = Box::new(Assign(var_name.clone(), node));
                node_list.push(assign_node);
                return Box::new(Var(var_name));
            }
            v @ Fixnum(..) | v @ Var(..) | v @ False | v @ True => {
                return Box::new(v);
            }
            Let { name, value, exp } => {
                let assign_node = Box::new(Assign(name, value));
                node_list.push(assign_node);
                return self.flattern_inner(exp, node_list);
            }
            If {
                cond,
                if_exp,
                else_exp,
            } => {
                let cond_var = self.flattern_inner(cond, node_list);
                let mut if_exps = Vec::new();
                let mut else_exps = Vec::new();
                let if_var = self.flattern_inner(if_exp, &mut if_exps);
                let else_var = self.flattern_inner(else_exp, &mut else_exps);
                let if_value_node = match (if_var.var(), else_var.var()) {
                    (Some(if_v), Some(else_v)) if if_v == else_v => if_var,
                    (Some(if_v), _) => {
                        else_exps.push(Box::new(Assign(if_v.to_owned(), else_var)));
                        if_var
                    }
                    (_, Some(else_v)) => {
                        if_exps.push(Box::new(Assign(else_v.to_owned(), if_var)));
                        else_var
                    }
                    _ => {
                        let v = self.var_allocator.alloc();
                        else_exps.push(Box::new(Assign(v.to_owned(), else_var)));
                        if_exps.push(Box::new(Assign(v.to_owned(), if_var)));
                        Box::new(Var(v.to_owned()))
                    }
                };
                let node = Box::new(Iff {
                    cond: cond_var,
                    if_exps,
                    else_exps,
                });
                node_list.push(node);
                if_value_node
            }
            val => {
                panic!("unexpected {:?}", val);
            }
        }
    }
}
