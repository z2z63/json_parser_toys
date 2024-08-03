use super::{Lexer, Result, Value};

struct LRParser<'s> {
    lexer: Lexer<'s>,
}

// CLOSURE(S') = {
//      S'->·S  S->·A  S->·B  S->·c  S->·d  S->·e  S->·f
//      A->·[SC]  A->·[]
//      B->·{DE}  B->·{}
// }

// CLOSURE(S) = {
//      S->·A  S->·B  S->·c  S->·d  S->·e  S->·f
//      A->·[SC]  A->·[]
//      B->·{DE}  B->·{}
// }
//
// CLOSURE(A) = {
//     A->·[SC]  A->·[]
// }
// CLOSURE(B) = {
//     B->·{DE}  B->·{}
// }
impl LRParser {
    pub fn new(lexer: Lexer) -> LRParser {
        LRParser {
            lexer
        }
    }
    // pub fn parse(&self) -> Result<Value> {}
}