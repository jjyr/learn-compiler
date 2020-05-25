use crate::ast::*;

pub fn flattern(node: Box<Node>) -> Vec<Box<Node>> {
    let mut context = Context::default();
    context.flattern_inner(node);
    context.assign_list
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
    assign_list: Vec<Box<Node>>,
    var_allocator: VarAlloc,
}

impl Context {
    fn flattern_inner(&mut self, node: Box<Node>) -> Box<Node> {
        use Value::*;

        let Node { token, value } = *node;
        match value {
            Program(sub_node) => {
                let sub_node = self.flattern_inner(sub_node);
                let node = Box::new(Node::new(Token::Program, Program(sub_node.clone())));
                self.assign_list.push(node);
                sub_node
            }
            Neg(sub_node) => {
                let sub_node = self.flattern_inner(sub_node);
                let neg_node = Box::new(Node::new(Token::Neg, Neg(sub_node)));
                let var_name = self.var_allocator.alloc();
                let assign_node =
                    Box::new(Node::new(Token::Assign, Assign(var_name.clone(), neg_node)));
                self.assign_list.push(assign_node);
                return Box::new(Node::new(Token::Var, Var(var_name)));
            }
            Read => {
                let node = Box::new(Node::new(Token::Read, Read));
                let var_name = self.var_allocator.alloc();
                let assign_node =
                    Box::new(Node::new(Token::Assign, Assign(var_name.clone(), node)));
                self.assign_list.push(assign_node);
                return Box::new(Node::new(Token::Var, Var(var_name)));
            }
            Add(lhs, rhs) => {
                // lhs is the next stmt after prev
                let lhs_var = self.flattern_inner(lhs);
                // lhs is the next next stmt after prev
                let rhs_var = self.flattern_inner(rhs);
                let var_name = self.var_allocator.alloc();
                let node = Box::new(Node::new(Token::Add, Add(lhs_var, rhs_var)));
                let assign_node =
                    Box::new(Node::new(Token::Assign, Assign(var_name.clone(), node)));
                self.assign_list.push(assign_node);
                return Box::new(Node::new(Token::Var, Var(var_name)));
            }
            v @ Fixnum(..) | v @ Var(..) => {
                return Box::new(Node::new(token, v));
            }
            Let(var_name, num, sub_node) => {
                let value_node = Box::new(Node::new(Token::Fixnum, Value::Fixnum(num)));
                let assign_node = Box::new(Node::new(
                    Token::Assign,
                    Value::Assign(var_name, value_node),
                ));
                self.assign_list.push(assign_node);
                return self.flattern_inner(sub_node);
            }
            val => {
                panic!("unexpected {:?}", val);
            }
        }
    }
}
