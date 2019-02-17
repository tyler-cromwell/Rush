extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let mut rl = Editor::<()>::new();
    //if rl.load_history("history.txt").is_err() {}

    loop {
        let input = rl.readline("rush$ ");
        match input {
            Ok(line) => {
                //rl.add_history_entry(line.as_ref());
            },
            Err(ReadlineError::Interrupted) => {break},
            Err(ReadlineError::Eof) => {break},
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    //rl.save_history("history.txt").unwrap();
}
