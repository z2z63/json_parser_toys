use std::collections::HashMap;
use super::{Lexer, Value, TOKEN, Result};


/// to LL1:
/// TOKEN: [ { } ] , : string number bool null
/// Value -> [ Array' | { Object' | string | number | bool | null
/// Array' -> ] | Value ValueList ]
/// ValueList -> , Value ValueList | ε
/// Object' -> } | Pair PairList }
/// PairList -> , Pair PairList | ε
/// Pair -> string : Value

/// | set    |     Value                          |     Array'                           |     Object'     |     ValueList | PairList | Pair     |
/// | :- :   |           :-:                      |     :-:                              |     :-:         |     :-:       |  :-:     |  :-:     |
/// | FIRST  |    \[\{ string number bool null    |    \]\[\{ string number bool null    |    \} string    |      , ε      |   , ε    |  string  |
/// | FOLLOW |     , # \] \}                      |     # , \]                           |     , # ] }     |     \]        |     \}   |  , \}    |
/// Value
/// - SELECT(Value -> [ Array') = [
/// - SELECT(Value -> { Object') = {
/// - SELECT(Value -> string) = string
/// - SELECT(Value -> number) = number
/// - SELECT(Value -> bool) = bool
/// - SELECT(Value -> null) = null
///
/// Array'
/// - SELECT(Array' -> ]) = ]
/// - SELECT(Array' -> Value ValueList]) = string number bool null [ {
///
/// ValueList
/// - SELECT(ValueList -> , Value ValueList) = ,
/// - SELECT(ValueList -> ε) = \emptyset and FOLLOW(ValueList) = ]
///
/// Object'
/// - SELECT(Object' -> }) = }
/// - SELECT(Object' -> Pair PairList}) = string
///
/// PairList
/// - SELECT(PairList -> , Pair PairList) = ,
/// - SELECT(PairList -> ε) = \emptyset and FOLLOW(PairList) = }
///
/// Pair
/// - SELECT(Pair -> string : Value) = string

pub struct DefiniteParser<'s> {
    lexer: Lexer<'s>,
}

impl<'s> DefiniteParser<'s> {
    pub fn new(lexer: Lexer) -> DefiniteParser {
        DefiniteParser {
            lexer
        }
    }

    pub fn parse(&mut self) -> Result<Value<'s>> {
        self.parse_value()
    }
    /// Value
    /// - SELECT(Value -> [ Array') = [
    /// - SELECT(Value -> { Object') = {
    /// - SELECT(Value -> string) = string
    /// - SELECT(Value -> number) = number
    /// - SELECT(Value -> bool) = bool
    /// - SELECT(Value -> null) = null
    fn parse_value(&mut self) -> Result<Value<'s>> {
        match self.lexer.lex()? {
            TOKEN::LBRACE => self.parse_object1(),
            TOKEN::LBRACKET => self.parse_array1(),
            TOKEN::STRING(s) => Ok(Value::String(s)),
            TOKEN::NUMBER(n) => Ok(Value::Number(n)),
            TOKEN::BOOL(b) => Ok(Value::Bool(b)),
            TOKEN::NULL => Ok(Value::Null),
            _ => Err(format!(
                "expect [ | {{ | string | number | bool | null at position: {}",
                self.lexer.index(),
            ))
        }
    }
    /// Array'
    /// - SELECT(Array' -> ]) = ]
    /// - SELECT(Array' -> Value ValueList]) = string number bool null [ {
    fn parse_array1(&mut self) -> Result<Value<'s>> {
        match self.lexer.lex()? {
            TOKEN::RBRACKET => Ok(Value::Array(vec![])),
            _ => {
                self.lexer.push_back();
                let value = self.parse_value()?;
                let mut list = self.parse_value_list()?;
                list.push(value);
                return if let TOKEN::RBRACKET = self.lexer.lex()? {
                    Ok(Value::Array(list))
                } else {
                    Err(format!("expect ] at position: {}", self.lexer.index()))
                };
            }
        }
    }
    /// Object'
    /// - SELECT(Object' -> }) = }
    /// - SELECT(Object' -> Pair PairList}) = string
    fn parse_object1(&mut self) -> Result<Value<'s>> {
        match self.lexer.lex()? {
            TOKEN::RBRACE => Ok(Value::Object(HashMap::new())),
            TOKEN::STRING(_) => {
                self.lexer.push_back();
                let pair = self.parse_pair()?;
                let mut list = self.parse_pair_list()?;
                list.push(pair);
                return if let TOKEN::RBRACE = self.lexer.lex()? {
                    Ok(Value::Object(HashMap::from_iter(list)))
                } else {
                    Err(format!("expect }} at position: {}", self.lexer.index()))
                };
            }
            _ => Err(format!("expect }} | string at position: {}", self.lexer.index()))
        }
    }
    /// Pair
    /// - SELECT(Pair -> string : Value) = string
    fn parse_pair(&mut self) -> Result<(&'s str, Value<'s>)> {
        if let TOKEN::STRING(s) = self.lexer.lex()? {
            if let TOKEN::COLON = self.lexer.lex()? {
                let value = self.parse_value()?;
                return Ok((s, value));
            }
            return  Err(format!("expect : at position: {}", self.lexer.index()))
        }
        return  Err(format!("expect string at position: {}", self.lexer.index()))
    }
    /// PairList
    /// - SELECT(PairList -> , Pair PairList) = ,
    /// - SELECT(PairList -> ε) = \emptyset and FOLLOW(PairList) = }
    fn parse_pair_list(&mut self) -> Result<Vec<(&'s str, Value<'s>)>> {
        match self.lexer.lex()? {
            TOKEN::COMMA => {
                let value = self.parse_pair()?;
                let mut list = self.parse_pair_list()?;
                list.push(value);
                Ok(list)
            }
            TOKEN::RBRACE => {
                self.lexer.push_back();
                Ok(vec![])
            }
            _ => {
                Err(format!("expect , | }} at position: {}", self.lexer.index()))
            }
        }
    }
    /// ValueList
    /// - SELECT(ValueList -> , Value ValueList) = ,
    /// - SELECT(ValueList -> ε) = \emptyset and FOLLOW(ValueList) = ]
    fn parse_value_list(&mut self) -> Result<Vec<Value<'s>>> {
        match self.lexer.lex()? {
            TOKEN::COMMA => {
                let value = self.parse_value()?;
                let mut list = self.parse_value_list()?;
                list.push(value);
                Ok(list)
            }
            TOKEN::RBRACKET => {
                self.lexer.push_back();
                Ok(vec![])
            }
            _ => Err(format!("expect , | ] at position: {}", self.lexer.index()))
        }
    }
}