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
pub type List = Vec<Expr>; // Maybe use VecDeque
pub type SExpr = Vec<Expr>;

// Lexical/Syntax

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

// --------

// Semantic

/// (def abc "hello")
/// should have only two args
struct Def {
    name: Symbol,
    value: Box<Expr2>,
}

/// (fn (x y) (+ x y))
/// can have multiple expr2 at the end, the last one is the one that matters, aka the one that
/// returns when the function ends
struct Lambda {
    args: Vec<Expr2>,
    // Can be ignored at first, always None
    docs: Option<String>,
    body: Vec<Expr2>,
}

/// (+ x y)
/// (add-one 5)
/// 1 symbol + N args (list of expr2)
struct FnCall {
    name: Symbol,
    args: Vec<Expr2>,
}

type Map2 = BTreeMap<Expr2, Expr2>;
type List2 = Vec<Expr2>; // Maybe use VecDeque

/// Basically just like Expr, but with meaning to the SExpressions, now they are either: 'def's,
/// 'lambda's or 'function call's. \o/
enum Expr2 {
    Int(Int),
    Float(Float),
    Bool(Bool),
    Character(Character),
    Keyword(Keyword),
    Symbol(Symbol),
    String(String),
    Map(Map2),
    List(List2),
    Def(Def),
    Lambda(Lambda),
    FnCall(FnCall),
}

type Program2 = Vec<Expr2>;


// --------------

// Program itself (env/context)

/// this will be very hard, we'll need to navigate through variables up like a linked list, context
/// by context until we give an "undefined var/def"
struct State {
    // some kind of var map, that can go up somehow

    // later for opt, consider compiling those before.
    // some symbol:lambda built in vars/fns/consts
    // - if/else
    // - first
    // - and
    // - etc...
}
