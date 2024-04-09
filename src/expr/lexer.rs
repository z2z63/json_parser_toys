use std::str::from_utf8;
use crate::expr::TOKEN;

pub struct Lexer<'s> {
    expr: &'s [u8],
    index: usize,
    current_token_size: usize,
}


// E -> E + E | E - E | E * E | E / E | (E) | NUMBER
// priority: () > */ > +-
// terminate symbol: NUMBER, +-*/()

impl Lexer<'_> {
    pub fn new(expr: &str) -> Lexer {
        Lexer {
            expr: expr.as_bytes(),
            index: 0,
            current_token_size: 0,
        }
    }
    pub fn lex(&mut self) -> Option<TOKEN> {
        if self.index >= self.expr.len() {
            return None;
        }
        const BLANK: [u8; 4] = [b' ', b'\n', b'\t', b'\r'];
        while BLANK.contains(&self.expr[self.index]) {
            self.index += 1;
        }
        self.current_token_size = self.index;
        let ret = match self.expr[self.index] {
            b'(' => {
                self.index += 1;
                TOKEN::LPAREN
            }
            b')' => {
                self.index += 1;
                TOKEN::RPAREN
            }
            b'+' => {
                self.index += 1;
                TOKEN::ADD
            }
            b'-' => {
                self.index += 1;
                TOKEN::SUB
            }
            b'*' => {
                self.index += 1;
                TOKEN::MUL
            }
            b'/' => {
                self.index += 1;
                TOKEN::DIV
            }
            _ if self.expr[self.index].is_ascii_digit() => {
                let start = self.index;
                while self.index < self.expr.len() && (self.expr[self.index].is_ascii_digit() || self.expr[self.index] == b'.') {
                    self.index += 1;
                }
                let end = self.index;
                let err_msg = format!("invalid number: {:?}", &self.expr[start..end]);
                let num = from_utf8(&self.expr[start..end])
                    .expect(&err_msg)
                    .parse::<f64>().expect(&err_msg);
                TOKEN::NUMBER(num)
            }
            _ => return None,
        };
        return Some(ret);
    }
}