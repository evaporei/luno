use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub type Int = i64;
pub type Float = OrderedFloat<f64>;
pub type Bool = bool;
pub type Character = char;
pub type Keyword = String;
pub type Symbol = String;
// String is the Rust one;
pub type Map = BTreeMap<Expr, Expr>;
pub type List = Vec<Expr>;
pub type SExpr = Vec<Expr>;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Expr {
    Int(Int),
    Float(Float),
    Bool(Bool),
    Character(Character),
    Keyword(Keyword),
    Symbol(Symbol),
    String(String),
    Map(Map),
    List(List),
    SExpr(SExpr),
}

pub type Program = Vec<Expr>;
