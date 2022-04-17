mod state;
mod value;

pub use state::State;
pub use value::Value;

/// Creates a Sol state, executes a line then kills it.
// lua_dostring
pub fn exec_line(line: &str) {
    let state = state::State::new();
    state.load_std();

    state.load_line(line);
    state.call();
}
