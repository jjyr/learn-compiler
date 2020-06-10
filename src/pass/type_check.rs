use crate::ast::*;
use std::collections::HashMap;
use std::result::Result;

fn expect_type_eq(expected: Type, actual: Type) -> Result<(), String> {
    if expected != actual {
        return Err(format!(
            "Incorrect type: expected {:?} actual {:?}",
            expected, actual
        ));
    }
    Ok(())
}

fn type_check_node(node: Box<Node>, var_types: &mut HashMap<String, Type>) -> Result<Type, String> {
    use Node::*;

    let t = match *node {
        Program(exp) => type_check_node(exp, var_types)?,
        Fixnum(_) => Type::Fixnum,
        True | False => Type::Boolean,
        Var(name) => *var_types.get(&name).expect("unknown"),
        Read => Type::Fixnum,
        Let { name, value, exp } => {
            let t = type_check_node(value, var_types)?;
            var_types.insert(name, t);
            type_check_node(exp, var_types)?
        }
        Not(exp) => {
            let t = type_check_node(exp, var_types)?;
            expect_type_eq(Type::Boolean, t)?;
            t
        }
        Add(lhs, rhs) => {
            let lhs_t = type_check_node(lhs, var_types)?;
            let rhs_t = type_check_node(rhs, var_types)?;
            expect_type_eq(Type::Fixnum, lhs_t)?;
            expect_type_eq(Type::Fixnum, rhs_t)?;
            Type::Fixnum
        }
        Eq(lhs, rhs) | Lt(lhs, rhs) | Lte(lhs, rhs) | Gt(lhs, rhs) | Gte(lhs, rhs) => {
            let lhs_t = type_check_node(lhs, var_types)?;
            let rhs_t = type_check_node(rhs, var_types)?;
            expect_type_eq(lhs_t, rhs_t)?;
            Type::Boolean
        }
        If {
            cond,
            mut if_exps,
            mut else_exps,
        } => {
            assert_eq!(if_exps.len(), 1);
            assert_eq!(else_exps.len(), 1);
            let cond_t = type_check_node(cond, var_types)?;
            expect_type_eq(cond_t, Type::Boolean)?;
            let if_t = type_check_node(if_exps.remove(0), var_types)?;
            let else_t = type_check_node(else_exps.remove(0), var_types)?;
            expect_type_eq(if_t, else_t)?;
            if_t
        }
        e => panic!("unexpected {:?}", e),
    };
    Ok(t)
}

pub fn type_check(exp: Box<Node>, info: &mut Info) -> Result<Type, String> {
    type_check_node(exp, &mut info.vars_types)
}
