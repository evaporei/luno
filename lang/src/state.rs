pub struct State;

impl State {
    /// Initiates a Sol state.
    // luaL_newstate
    pub fn new() -> Self {
        Self
    }

    /// Loads the Sol's standard library into the State.
    // luaL_openlibs
    pub fn load_std(&self) {
    }

    /// Compiles a line.
    // luaL_loadstring
    pub fn load_line(&self, _line: &str) {
    }

    /// Calls the compiled function.
    // lua_pcall
    pub fn call(&self) {
    }
}

// FIXME: Perhaps this can be removed later.
// lua_close
impl Drop for State {
    fn drop(&mut self) {
    }
}
