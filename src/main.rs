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

                if tokens[0] == "exit" {
                    break;
                }

                let result = Command::new(tokens[0])
                    .args(tokens[1..tokens.len()].into_iter())
                    .output();

                match result {
                    Ok(result) => {
                        stdout().write_all(&result.stdout).unwrap();
                        stderr().write_all(&result.stderr).unwrap();
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {},
            Err(ReadlineError::Eof) => {break},
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(history).unwrap();
}
