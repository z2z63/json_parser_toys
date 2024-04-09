use std::ops::Index;
use super::Result;

#[derive(Debug)]
#[repr(u8)]
pub enum TOKEN<'s> {
    LBRACE = 0,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    COLON,
    STRING(&'s str),
    NUMBER(f64),
    BOOL(bool),
    NULL,
}

pub struct Lexer<'s> {
    json_str: &'s [u8],
    index: usize,
    current_token_size: usize,
}


impl<'s> Lexer<'s> {
    pub fn new(json_str: &'s str) -> Lexer<'s> {
        Lexer {
            json_str: json_str.as_bytes(),
            index: 0,
            current_token_size: 0,
        }
    }

    fn eof_msg(&self) -> String {
        format!("unexpected end of file at position {}", self.index + 1)
    }

    pub fn lex(&mut self) -> Result<TOKEN<'s>> {
        if self.index >= self.json_str.len() {
            return Err(self.eof_msg());
        }
        let blank = [b' ', b'\n', b'\t', b'\r'];
        while blank.contains(self.json_str.get(self.index).ok_or(self.eof_msg())?) {
            self.index += 1;
        }
        self.current_token_size = self.index;
        let ret = match self.json_str.get(self.index).ok_or(self.eof_msg())? {
            b'{' => {
                self.index += 1;
                TOKEN::LBRACE
            }
            b'}' => {
                self.index += 1;
                TOKEN::RBRACE
            }
            b'[' => {
                self.index += 1;
                TOKEN::LBRACKET
            }
            b']' => {
                self.index += 1;
                TOKEN::RBRACKET
            }
            b',' => {
                self.index += 1;
                TOKEN::COMMA
            }
            b':' => {
                self.index += 1;
                TOKEN::COLON
            }
            b'"' => {
                self.index += 1;
                let start = self.index;
                while !(self.json_str[self.index] == b'"' && (self.index == 0 || self.json_str[self.index - 1] != b'\\')) {
                    self.index += 1;
                }
                let end = self.index;
                self.index += 1;
                TOKEN::STRING(std::str::from_utf8(&self.json_str[start..end]).unwrap())
            }
            value if value.is_ascii_digit() => {
                let start = self.index;
                while self.json_str[self.index].is_ascii_digit() || self.json_str[self.index] == b'.' {
                    self.index += 1;
                }
                let end = self.index;
                TOKEN::NUMBER(std::str::from_utf8(&self.json_str[start..end]).unwrap().parse().unwrap())
            }
            _ if self.json_str[self.index..self.index + 4].as_ref() == b"true" => {
                self.index += 4;
                TOKEN::BOOL(true)
            }
            _ if self.json_str[self.index..self.index + 5].as_ref() == b"false".as_ref() => {
                self.index += 5;
                TOKEN::BOOL(false)
            }
            _ if self.json_str[self.index..self.index + 4].as_ref() == b"null".as_ref() => {
                self.index += 4;
                TOKEN::NULL
            }
            _ => return Err(format!("unexpected token at position {}", self.index))
        };
        self.current_token_size = self.index - self.current_token_size;
        return Ok(ret);
    }

    pub fn push_back(&mut self) {
        self.index -= self.current_token_size;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}