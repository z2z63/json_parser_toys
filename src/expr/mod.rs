mod lexer;
type Result<T> = std::result::Result<T, String>;
#[derive(Debug)]
pub enum TOKEN {
    LPAREN,
    RPAREN,
    ADD,
    SUB,
    MUL,
    DIV,
    NUMBER(f64),
}

pub enum Expr {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Number(f64),
}

// E -> A + A | A - A
// A -> B * B | B / B
// A -> (E) | NUMBER
// B -> (E) | NUMBER


struct OPGParser<'a> {
    lexer: lexer::Lexer<'a>,
    stack: Vec<TOKEN>,
    expr_stack: Vec<Expr>,
}

impl OPGParser<'_> {
    pub fn new(lexer: lexer::Lexer) -> OPGParser {
        OPGParser {
            lexer,
            stack: vec![TOKEN::LPAREN],
            expr_stack: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {}
}