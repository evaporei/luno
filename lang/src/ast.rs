pub type Int = i64;
pub type Float = f64;
pub type Bool = bool;
pub type Character = char;
pub type Keyword = String;
pub type Symbol = String;
// String is the Rust one;
pub type SExpr = Vec<Expr>;

#[derive(Debug)]
pub enum Expr {
    Int(Int),
    Float(Float),
    Bool(Bool),
    Character(Character),
    Keyword(Keyword),
    Symbol(Symbol),
    String(String),
    SExpr(SExpr),
}

pub type Program = Vec<Expr>;
