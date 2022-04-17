mod ast;
mod parser;
mod state;

pub use parser::Parser;
pub use state::State;

/// Creates a Sol state, executes a line then kills it.
// lua_dostring
pub fn exec_line(line: &str) {
    let state = state::State::new();
    state.load_std();

    state.load_line(line);
    state.call();
}
