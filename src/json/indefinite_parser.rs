use std::collections::HashMap;
use super::{Lexer, Result, TOKEN, Value};

// Value -> Array | Object | String | Number | Bool | Null
// Array -> [Value ValueList]
// Array -> []
// ValueList -> , Value ValueList
// ValueList -> ε
// Object -> { Pair PairList }
// Object -> {}
// PairList -> , Pair PairList
// PairList -> ε
// Pair -> String : Value
pub struct IndefiniteParser<'s> {
    lexer: Lexer<'s>,
}

impl<'s> IndefiniteParser<'s> {
    pub fn new(lexer: Lexer<'s>) -> IndefiniteParser<'s> {
        IndefiniteParser {
            lexer,
        }
    }

    pub fn parse(&mut self) -> Result<Value<'s>> {
        self.parse_value()
    }

    // Value -> Array | Object | String | Number | Bool | Null
    fn parse_value(&mut self) -> Result<Value<'s>> {
        let token = self.lexer.lex()?;
        return match token {
            TOKEN::LBRACE => {
                self.lexer.push_back();
                self.parse_object()
            }
            TOKEN::LBRACKET => {
                self.lexer.push_back();
                self.parse_array()
            }
            TOKEN::STRING(s) => Ok(Value::String(s)),
            TOKEN::NUMBER(n) => Ok(Value::Number(n)),
            TOKEN::BOOL(b) => Ok(Value::Bool(b)),
            TOKEN::NULL => Ok(Value::Null),
            _ => Err(format!("expect [ | {{ | string | number | bool | null at position: {}", self.lexer.index())),
        };
    }
    // Array -> [Value ValueList]
    // Array -> []
    fn parse_array(&mut self) -> Result<Value<'s>> {
        if let TOKEN::LBRACKET = self.lexer.lex()? {
            let mut list = vec![];
            if let Ok(value) = self.parse_value() {
                list = self.parse_value_list()?;
                list.push(value);
            } else {
                self.lexer.push_back();
            }
            return if let TOKEN::RBRACKET = self.lexer.lex()? {
                Ok(Value::Array(list))
            } else {
                Err(format!("expect ] at position: {}", self.lexer.index()))
            };
        }
        Err(format!("expect [ at position: {}", self.lexer.index()))
    }

    // ValueList -> , Value ValueList
    // ValueList -> ε
    fn parse_value_list(&mut self) -> Result<Vec<Value<'s>>> {
        return if let TOKEN::COMMA = self.lexer.lex()? {
            let value = self.parse_value()?;
            let mut list = self.parse_value_list()?;
            list.push(value);
            Ok(list)
        } else {
            self.lexer.push_back();
            Ok(vec![])
        };
    }

    // Object -> { Pair PairList }
    // Object -> {}
    fn parse_object(&mut self) -> Result<Value<'s>> {
        if let TOKEN::LBRACE = self.lexer.lex()? {
            let mut list = vec![];
            if let Ok(pair) = self.parse_pair() {
                list = self.parse_pair_list()?;
                list.push(pair);
            } else {
                self.lexer.push_back();
            }
            return if let TOKEN::RBRACE = self.lexer.lex()? {
                Ok(Value::Object(HashMap::from_iter(list)))
            } else {
                Err(format!("expect }} at position: {}", self.lexer.index()))
            };
        }
        return Err(format!("expect {{ at position: {}", self.lexer.index()));
    }

    // PairList -> , Pair PairList
    // PairList -> ε
    fn parse_pair_list(&mut self) -> Result<Vec<(&'s str, Value<'s>)>> {
        return if let TOKEN::COMMA = self.lexer.lex()? {
            let pair = self.parse_pair()?;
            let mut list = self.parse_pair_list()?;
            list.push(pair);
            Ok(list)
        } else {
            self.lexer.push_back();
            Ok(vec![])
        };
    }

    // Pair -> String : Value
    fn parse_pair(&mut self) -> Result<(&'s str, Value<'s>)> {
        if let TOKEN::STRING(s) = self.lexer.lex()? {
            if let TOKEN::COLON = self.lexer.lex()? {
                let value = self.parse_value()?;
                return Ok((s, value));
            } else {
                Err(format!("expect : at position: {}", self.lexer.index()))
            }
        } else {
            self.lexer.push_back();
            Err(format!("expect string at position: {}", self.lexer.index()))
        }
    }
}