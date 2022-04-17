// TODO:
// 1. add GC
// 2. optimize number/bool to be on stack
pub enum Value {
    Int(u32),
    // TODO: maybe some kind of bigint later?
    Float(f64),
    // Maybe use a shorter name.
    String(String),
    Bool(bool),
    // This should only need a number instead of a whole string.
    // Maybe it can be converted in compilation time.
    Atom(String),
}
