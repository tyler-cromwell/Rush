use std::io::*;
use std::process::Command;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let history = ".rush_history";
    let mut rl = Editor::<()>::new();
    if rl.load_history(history).is_err() {}

    loop {
        let input = rl.readline("rush$ ");
        match input {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                let tokens = line.split(" ");
                let tokens: Vec<&str> = tokens.collect();
                let output = Command::new(tokens[0])
                    .args(tokens[1..tokens.len()].into_iter())
                    .output()
                    .expect("failed to execute process");
                stdout().write_all(&output.stdout).unwrap();
            },
            //Err(ReadlineError::Interrupted) => {break},
            Err(ReadlineError::Eof) => {break},
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(history).unwrap();
}
