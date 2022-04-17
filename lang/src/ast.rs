pub type Int = i64;

pub type Float = f64;

pub type Bool = bool;

pub type SExpr = Vec<Expr>;

#[derive(Debug)]
pub enum Expr {
    Int(Int),
    Float(Float),
    Bool(Bool),
    SExpr(SExpr),
}

pub type Program = Vec<Expr>;
