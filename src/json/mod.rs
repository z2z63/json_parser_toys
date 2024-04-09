use std::collections::HashMap;
use std::fmt::{Display, format, Formatter};

pub mod indefinite_parser;
pub mod definite_parser;
pub mod table_driven_parser;
pub mod lexer;
mod lr_parser;

pub use lexer::{Lexer, TOKEN};
pub use indefinite_parser::IndefiniteParser;
pub use definite_parser::DefiniteParser;
pub use table_driven_parser::TableDrivenParser;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub enum Value<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Null,
    Object(HashMap<&'a str, Value<'a>>),
    Array(Vec<Value<'a>>),
}

impl Value<'_> {
    fn fmt_value(&self, f: &mut Formatter<'_>, width: usize) -> std::fmt::Result {
        match self {
            Value::String(_) => self.fmt_string(f),
            Value::Number(_) => self.fmt_number(f),
            Value::Bool(_) => self.fmt_bool(f),
            Value::Null => self.fmt_null(f),

            Value::Array(arr) => self.fmt_array(f, width),
            Value::Object(obj) => self.fmt_object(f, width),
        }
    }

    fn fmt_string(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            _ => panic!("invalid token"),
        }
    }

    fn fmt_number(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            _ => panic!("invalid token"),
        }
    }

    fn fmt_bool(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            _ => panic!("invalid token"),
        }
    }

    fn fmt_null(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            _ => panic!("invalid token"),
        }
    }

    fn fmt_array(&self, f: &mut Formatter<'_>, width: usize) -> std::fmt::Result {
        match self {
            Value::Array(arr) => {
                write!(f, "[\n")?;
                for i in 0..arr.len() {
                    Self::tab(f, width + 1)?;
                    arr[i].fmt_value(f, width + 1)?;
                    if (i < arr.len() - 1) {
                        write!(f, ",\n")?;
                    }
                }
                write!(f, "\n")?;
                Self::tab(f, width)?;
                write!(f, "]")
            }
            _ => panic!("invalid token"),
        }
    }

    fn fmt_object(&self, f: &mut Formatter<'_>, width: usize) -> std::fmt::Result {
        match self {
            Value::Object(obj) => {
                write!(f, "{{\n")?;
                let mut iter = obj.iter().peekable();
                while let Some((key, value)) = iter.next() {
                    Self::tab(f, width + 1)?;
                    write!(f, "\"{}\": ", key)?;
                    value.fmt_value(f, width + 1)?;
                    if iter.peek().is_some() {
                        write!(f, ",\n")?;
                    }
                }
                write!(f, "\n")?;
                Self::tab(f, width)?;
                write!(f, "}}")
            }
            _ => panic!("invalid token"),
        }
    }

    fn tab(f: &mut Formatter<'_>, width: usize) -> std::fmt::Result {
        for _ in 0..width {
            write!(f, "{}", TAB)?;
        }
        Ok(())
    }

    fn index(&self, index: usize) -> Option<&Value> {
        match self {
            Value::Array(arr) => {
                arr.get(index)
            }
            _ => panic!("{}", format!("expect array, found {:?}", self))
        }
    }
    fn get(&self, index: &str) -> Option<&Value> {
        match self {
            Value::Object(arr) => {
                arr.get(index)
            }
            _ => panic!("{}", format!("expect object, found {:?}", self))
        }
    }
}

const TAB: &str = "    ";

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_value(f, 0)
    }
}

impl<'a> std::ops::Index<usize> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(arr) => {
                &arr[index]
            }
            _ => panic!("{}", format!("expect array, found {:?}", self))
        }
    }
}

impl<'a> std::ops::Index<&str> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Object(arr) => {
                &arr[index]
            }
            _ => panic!("{}", format!("expect object, found {:?}", self))
        }
    }
}

impl<'a> AsRef<str> for Value<'a> {
    fn as_ref(&self) -> &'a str {
        match self {
            Value::String(s) => s,
            _ => panic!("{}", format!("expect string, found {:?}", self)),
        }
    }
}

impl AsRef<f64> for Value<'_> {
    fn as_ref(&self) -> &f64 {
        match self {
            Value::Number(n) => n,
            _ => panic!("{}", format!("expect number, found {:?}", self)),
        }
    }
}

impl AsRef<bool> for Value<'_> {
    fn as_ref(&self) -> &bool {
        match self {
            Value::Bool(b) => b,
            _ => panic!("{}", format!("expect bool, found {:?}", self)),
        }
    }
}

impl<'a> AsRef<HashMap<&'a str, Value<'a>>> for Value<'a> {
    fn as_ref(&self) -> &HashMap<&'a str, Value<'a>> {
        match self {
            Value::Object(obj) => obj,
            _ => panic!("{}", format!("expect object, found {:?}", self)),
        }
    }
}

impl<'a> AsRef<Vec<Value<'a>>> for Value<'a> {
    fn as_ref(&self) -> &Vec<Value<'a>> {
        match self {
            Value::Array(arr) => arr,
            _ => panic!("{}", format!("expect array, found {:?}", self)),
        }
    }
}