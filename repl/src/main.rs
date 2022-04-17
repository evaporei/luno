use rustyline::error::ReadlineError;
use rustyline::Editor;
use sol_lang::State as SolState;

fn main() {
    // TODO: add completer by replacing `()`
    let mut rl = Editor::<()>::new();
    // TODO: make history more robust?
    let _ = rl.load_history("history.txt");

    let state = SolState::new();
    state.load_std();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                state.load_line(&line);
                state.call();
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
