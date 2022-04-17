use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // TODO: add completer by replacing `()`
    let mut rl = Editor::<()>::new();
    // TODO: make history more robust?
    let _ = rl.load_history("history.txt");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
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
