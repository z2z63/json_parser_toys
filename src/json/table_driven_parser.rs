use std::collections::HashMap;
use super::{TOKEN, Value, Result, Lexer};

// TOKEN: [ { } ] , : string number bool null
// Value -> [ Array' | { Object' | string | number | bool | null
// Array' -> ] | Value ValueList ]
// ValueList -> , Value ValueList | ε
// Object' -> } | Pair PairList }
// PairList -> , Pair PairList | ε
// Pair -> string : Value
mod SYMBOLS {
    pub const Value: u8 = 0;
    pub const Array1: u8 = 1;
    pub const ValueList: u8 = 2;
    pub const Object1: u8 = 3;
    pub const PairList: u8 = 4;
    pub const Pair: u8 = 5;

    pub const LBRACE: u8 = 6;
    pub const RBRACE: u8 = 7;
    pub const LBRACKET: u8 = 8;
    pub const RBRACKET: u8 = 9;
    pub const COMMA: u8 = 10;
    pub const COLON: u8 = 11;
    pub const STRING: u8 = 12;
    pub const NUMBER: u8 = 13;
    pub const BOOL: u8 = 14;
    pub const NULL: u8 = 15;
    pub const EPSILON: u8 = 16;

    pub const STRING_TABLE: [&str; 17] = [
        "Value",
        "Array1",
        "ValueList",
        "Object1",
        "PairList",
        "Pair",
        "{",
        "}",
        "[",
        "]",
        ",",
        ":",
        "string",
        "number",
        "bool",
        "null",
        "epsilon",
    ];
}
// TABLE[NONTERMINAL][TERMINAL] = index of PRODUCTION
// COL:     Value Array1 ValueList Object1 PairList Pair
// ROW:     LBRACE  RBRACE LBRACKET RBRACKET COMMA COLON  STRING NUMBER BOOL NULL EPSILON

/// Value(0-5)
/// - SELECT(Value -> [ Array') = [
/// - SELECT(Value -> { Object') = {
/// - SELECT(Value -> string) = string
/// - SELECT(Value -> number) = number
/// - SELECT(Value -> bool) = bool
/// - SELECT(Value -> null) = null
///
/// Array'(6-7)
/// - SELECT(Array' -> ]) = ]
/// - SELECT(Array' -> Value ValueList]) = string number bool null [ {
///
/// ValueList(8-9)
/// - SELECT(ValueList -> , Value ValueList) = ,
/// - SELECT(ValueList -> ε) = \emptyset and FOLLOW(ValueList) = ]
///
/// Object'(10-11)
/// - SELECT(Object' -> }) = }
/// - SELECT(Object' -> Pair PairList}) = string
///
/// PairList(12-13)
/// - SELECT(PairList -> , Pair PairList) = ,
/// - SELECT(PairList -> ε) = \emptyset and FOLLOW(PairList) = }
///
/// Pair(14)
/// - SELECT(Pair -> string : Value) = string
///
/// predict table
///
/// | noterminal | LBRACE | RBRACE| LBRACKET |RBRACKET| COMMA |COLON | STRING| NUMBER| BOOL| NULL |EPSILON|
/// | ---------- |--------|-------|----------|--------|-------|------|-------|-------|-----|------|-------|
/// | Value      |   1    |   -1  |    0     |   -1   |   -1  |  -1  |   2   |   3   |  4  |   5  |   -1  |
/// | Array1     |   7    |   -1  |    7     |    6   |   -1  |  -1  |   7   |   7   |  7  |   7  |   -1  |
/// | ValueList  |  -1    |   -1  |   -1     |    9   |    8  |  -1  |  -1   |  -1   | -1  |  -1  |   -1  |
/// | Object1    |  -1    |   10  |   -1     |   -1   |   -1  |  -1  |  11   |  -1   | -1  |  -1  |   -1  |
/// | PairList   |  -1    |   13  |   -1     |   -1   |   12  |  -1  |  -1   |  -1   | -1  |  -1  |   -1  |
/// | Pair       |  -1    |   -1  |   -1     |   -1   |   -1  |  -1  |  14   |  -1   | -1  |  -1  |   -1  |
const TABLE: [[i8; 10]; 6] = [
//                LBRACE  RBRACE LBRACKET RBRACKET COMMA COLON  STRING NUMBER BOOL NULL EPSILON
    /*   Value */    [1, -1, 0, -1, -1, -1, 2, 3, 4, 5],
    /* Array1  */    [7, -1, 7, 6, -1, -1, 7, 7, 7, 7],
    /*ValueList*/    [-1, -1, -1, 9, 8, -1, -1, -1, -1, -1],
    /*Object1  */    [-1, 10, -1, -1, -1, -1, 11, -1, -1, -1],
    /*PairList */    [-1, 13, -1, -1, 12, -1, -1, -1, -1, -1],
    /*  Pair  */     [-1, -1, -1, -1, -1, -1, 14, -1, -1, -1],
];

pub struct TableDrivenParser<'s> {
    lexer: Lexer<'s>,
    symbol_stack: Vec<u8>,
    value_stack: Vec<Value<'s>>,
    rules_stack: Vec<i8>,
    PRODUCTION: [Vec<u8>; 15],
}

fn token2symbol(token: &TOKEN) -> u8 {
    match token {
        TOKEN::LBRACE => SYMBOLS::LBRACE,
        TOKEN::RBRACE => SYMBOLS::RBRACE,
        TOKEN::LBRACKET => SYMBOLS::LBRACKET,
        TOKEN::RBRACKET => SYMBOLS::RBRACKET,
        TOKEN::COMMA => SYMBOLS::COMMA,
        TOKEN::COLON => SYMBOLS::COLON,
        TOKEN::STRING(_) => SYMBOLS::STRING,
        TOKEN::NUMBER(_) => SYMBOLS::NUMBER,
        TOKEN::BOOL(_) => SYMBOLS::BOOL,
        TOKEN::NULL => SYMBOLS::NULL,
    }
}

fn is_terminal(symbol: u8) -> bool {
    return symbol >= 6 && symbol <= 16;
}

impl<'s> TableDrivenParser<'s> {
    pub fn new(lexer: Lexer) -> TableDrivenParser {
        TableDrivenParser {
            lexer,
            symbol_stack: vec![SYMBOLS::Value],
            value_stack: vec![],
            rules_stack: vec![],
            PRODUCTION: [
                vec![SYMBOLS::Value, SYMBOLS::LBRACKET, SYMBOLS::Array1],
                vec![SYMBOLS::Value, SYMBOLS::LBRACE, SYMBOLS::Object1],
                vec![SYMBOLS::Value, SYMBOLS::STRING],
                vec![SYMBOLS::Value, SYMBOLS::NUMBER],
                vec![SYMBOLS::Value, SYMBOLS::BOOL],
                vec![SYMBOLS::Value, SYMBOLS::NULL],
                vec![SYMBOLS::Array1, SYMBOLS::RBRACKET],
                vec![SYMBOLS::Array1, SYMBOLS::Value, SYMBOLS::ValueList, SYMBOLS::RBRACKET],
                vec![SYMBOLS::ValueList, SYMBOLS::COMMA, SYMBOLS::Value, SYMBOLS::ValueList],
                vec![SYMBOLS::ValueList, SYMBOLS::EPSILON],
                vec![SYMBOLS::Object1, SYMBOLS::RBRACE],
                vec![SYMBOLS::Object1, SYMBOLS::Pair, SYMBOLS::PairList, SYMBOLS::RBRACE],
                vec![SYMBOLS::PairList, SYMBOLS::COMMA, SYMBOLS::Pair, SYMBOLS::PairList],
                vec![SYMBOLS::PairList, SYMBOLS::EPSILON],
                vec![SYMBOLS::Pair, SYMBOLS::STRING, SYMBOLS::COLON, SYMBOLS::Value],
            ],
        }
    }
    pub fn parse(&mut self) -> Result<Value<'s>> {
        while let Ok(token) = self.lexer.lex() {
            let symbol = token2symbol(&token);
            while let Some(expected) = self.symbol_stack.pop() {
                if is_terminal(expected) {
                    if expected == symbol {
                        self.push_value(token);
                        break;
                    } else if expected == SYMBOLS::EPSILON {
                        continue;
                    } else {
                        let msg = format!(
                            "expect {} at position: {}, found {}",
                            SYMBOLS::STRING_TABLE[expected as usize],
                            self.lexer.index(), SYMBOLS::STRING_TABLE[symbol as usize]
                        );
                        return Err(msg);
                    }
                } else {
                    let index = TABLE[expected as usize][(symbol - 6) as usize];
                    if index == -1 {
                        let mut valid_symbols: Vec<&str> = vec![];
                        for i in 0..10 {
                            if TABLE[expected as usize][i] != -1 {
                                valid_symbols.push(SYMBOLS::STRING_TABLE[6 + i]);
                            }
                        }
                        let msg = format!(
                            "expect {} at position: {}, found {}",
                            valid_symbols.join(" | "),
                            self.lexer.index(),
                            SYMBOLS::STRING_TABLE[6 + symbol as usize],
                        );
                        return Err(msg);
                    } else {
                        let production = &self.PRODUCTION[index as usize];
                        for i in (1..production.len()).rev() {
                            self.symbol_stack.push(production[i]);
                        }
                        self.rules_stack.push(index);
                    }
                }
            }
        }
        if (self.value_stack.len() != 1) {
            panic!("---")
        } else {
            let value = self.value_stack.pop().unwrap();
            return Ok(value);
        }
    }

    fn push_value(&mut self, token: TOKEN<'s>) -> () {
        match token {
            TOKEN::STRING(s) => {
                match self.rules_stack.pop().unwrap() {
                    14 | 2 => self.value_stack.push(Value::String(s)),
                    _ => panic!("unexpected production")
                }
            }
            TOKEN::NUMBER(n) => {
                match self.rules_stack.pop().unwrap() {
                    3 => self.value_stack.push(Value::Number(n)),
                    _ => panic!("unexpected production")
                }
            }
            TOKEN::BOOL(b) => {
                match self.rules_stack.pop().unwrap() {
                    4 => self.value_stack.push(Value::Bool(b)),
                    _ => panic!("unexpected production")
                }
            }
            TOKEN::NULL => {
                match self.rules_stack.pop().unwrap() {
                    5 => self.value_stack.push(Value::Null),
                    _ => panic!("unexpected production")
                }
            }
            TOKEN::RBRACE => {
                let mut map = HashMap::new();
                self.rules_stack.pop();
                while [11, 12].contains(&self.rules_stack.pop().unwrap())  {
                    // 13 12 12 12 12 ... 11
                    if let value = self.value_stack.pop().unwrap() {
                        if let Value::String(s) = self.value_stack.pop().unwrap() {
                            map.insert(s, value);
                        }
                    }
                }
                self.value_stack.push(Value::Object(map));
            }
            TOKEN::RBRACKET => {
                let mut list = vec![];
                self.rules_stack.pop();
                // 9 8 8 8 8 8 8 ... 7
                while [8, 7].contains(&self.rules_stack.pop().unwrap()) {
                    let value = self.value_stack.pop().unwrap();
                    list.insert(0, value);
                }
                self.value_stack.push(Value::Array(list));
            }
            _ => {}
        };
    }
}


