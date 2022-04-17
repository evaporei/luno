pub type Character = char;

pub type Int = i64;

pub type Float = f64;

pub type Bool = bool;

pub type SExpr = Vec<Expr>;

#[derive(Debug)]
pub enum Expr {
    String(String),
    Character(Character),
    Int(Int),
    Float(Float),
    Bool(Bool),
    SExpr(SExpr),
}

pub type Program = Vec<Expr>;
