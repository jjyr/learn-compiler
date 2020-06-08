use crate::ast::{Node, Token};
use std::iter::FromIterator;

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
        let end_at = std::cmp::min(self.source.len(), self.cur + expected.len());
        let actual = &self.source[self.cur..end_at];
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
            'n' if self.match_str("not").is_ok() => Not,
            't' if self.match_str("true").is_ok() => True,
            'f' if self.match_str("false").is_ok() => False,
            '=' if self.match_str("==").is_ok() => Eq,
            '<' if self.match_str("<=").is_ok() => Lte,
            '<' => {
                self.cur += 1;
                Lt
            }
            '>' if self.match_str(">=").is_ok() => Gte,
            '>' => {
                self.cur += 1;
                Gt
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
        let node = match token {
            Program => Node::Program(self.read_exp()),
            Add => Node::Add(self.read_exp(), self.read_exp()),
            Neg => Node::Neg(self.read_exp()),
            Fixnum => Node::Fixnum(self.read_fixnum().expect("fixnum")),
            Read => Node::Read,
            Let => {
                self.expect_str("(");
                self.expect_str("[");
                let name = self.read_var().expect("var");
                self.next_char();
                let value = self.read_exp();
                match value.as_ref() {
                    Node::Fixnum(_) | Node::True | Node::False => {}
                    v => {
                        panic!("expect value, got {:?}", v);
                    }
                }
                self.expect_str("]");
                self.expect_str(")");
                Node::Let {
                    name,
                    value,
                    exp: self.read_exp(),
                }
            }
            Var => Node::Var(self.read_var().expect("var")),
            True => Node::True,
            False => Node::False,
            Not => Node::Not(self.read_exp()),
            Eq => Node::Eq(self.read_exp(), self.read_exp()),
            Lt => Node::Lt(self.read_exp(), self.read_exp()),
            Lte => Node::Lte(self.read_exp(), self.read_exp()),
            Gt => Node::Gt(self.read_exp(), self.read_exp()),
            Gte => Node::Gte(self.read_exp(), self.read_exp()),
        };
        if in_paren {
            self.expect_str(")");
        }
        return Box::new(node);
    }

    pub fn parse_program(&mut self) -> Box<Node> {
        self.read_exp()
    }
}
