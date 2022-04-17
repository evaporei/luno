#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
}

pub type Program = Vec<Expr>;
