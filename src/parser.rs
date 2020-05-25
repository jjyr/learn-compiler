use crate::ast::{Node, Token, Value};
use std::iter::FromIterator;

const MSG_LEN: usize = 8;

pub struct Parser {
    source: Vec<char>,
    cur: usize,
}

impl Parser {
    pub fn new(source: Vec<char>) -> Self {
        Parser { source, cur: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.cur == self.source.len() {
            return None;
        }
        while self.cur < self.source.len() && self.source[self.cur] == ' ' {
            self.cur += 1;
        }
        if self.cur == self.source.len() {
            return None;
        }
        Some(self.source[self.cur])
    }

    fn match_str(&mut self, expected: &str) -> Result<(), String> {
        self.next_char();
        let actual = &self.source[self.cur..self.cur + expected.len()];
        let matched = actual == &expected.chars().collect::<Vec<_>>()[..];
        if matched {
            self.cur += expected.len();
            Ok(())
        } else {
            Err(String::from_iter(actual.iter()))
        }
    }

    fn expect_str(&mut self, expected: &str) {
        if let Err(actual) = self.match_str(expected) {
            panic!(
                "parse token error at {}, expected {}, but got unexpected token: '{}'",
                self.cur, expected, actual
            );
        }
    }

    fn read_fixnum(&mut self) -> Option<isize> {
        let prev_cur = self.cur;
        let mut num = 0;
        loop {
            let chr = self.source[self.cur];
            if chr > '9' || chr < '0' {
                break;
            }
            num = num * 10 + chr.to_digit(10).unwrap() as isize;
            self.cur += 1;
        }
        if prev_cur == self.cur {
            return None;
        }
        Some(num)
    }

    fn read_var(&mut self) -> Option<String> {
        let prev_cur = self.cur;
        let mut i = 0;
        let mut var = String::new();
        loop {
            let chr = self.source[self.cur];
            if chr > 'z' || chr < 'a' {
                break;
            }
            self.cur += 1;
            var.push(chr);
        }
        if prev_cur == self.cur {
            return None;
        }
        Some(var)
    }

    fn read_token(&mut self) -> Option<Token> {
        use Token::*;

        let chr = match self.next_char() {
            Some(chr) => chr,
            None => return None,
        };
        let token = match chr {
            'p' if self.match_str("program").is_ok() => Program,
            'l' if self.match_str("let").is_ok() => Let,
            '+' => {
                self.cur += 1;
                Add
            }
            '-' => {
                self.cur += 1;
                Neg
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Fixnum,
            'r' if self.match_str("read").is_ok() => Read,
            _ => Var,
        };
        Some(token)
    }

    fn read_exp(&mut self) -> Box<Node> {
        use Token::*;

        let in_paren = self.match_str("(").is_ok();
        let token = self.read_token().expect("token");
        let mut n = Node::new(token);
        match token {
            Program => {
                n.lhs = Some(self.read_exp());
            }
            Add => {
                n.lhs = Some(self.read_exp());
                n.rhs = Some(self.read_exp());
            }
            Neg => {
                n.lhs = Some(self.read_exp());
            }
            Fixnum => {
                n.value = Some(Value::Fixnum(self.read_fixnum().expect("fixnum")));
            }
            Read => {
                // only require the token
            }
            Let => {
                self.expect_str("(");
                self.expect_str("[");
                let var = self.read_var().expect("var");
                self.next_char();
                let bound_value = self.read_fixnum().expect("fixnum");
                n.value = Some(Value::Let(var, bound_value));
                self.expect_str("]");
                self.expect_str(")");
                n.lhs = Some(self.read_exp());
            }
            Var => {
                n.value = Some(Value::Var(self.read_var().expect("var")));
            }
            _ => {
                panic!("expected token, got {:?}", token);
            }
        }
        if in_paren {
            self.expect_str(")");
        }
        return Box::new(n);
    }

    pub fn parse_program(&mut self) -> Box<Node> {
        self.read_exp()
    }
}
