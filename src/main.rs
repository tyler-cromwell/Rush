use std::env;
use std::io::*;
use std::process::Command;

extern crate dirs;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let history = ".rush_history";
    let mut rl = Editor::<()>::new();
    if rl.load_history(history).is_err() {}

    loop {
        let input = rl.readline("rush$ ");

        let line = match input {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        };

        let line = line.trim();
        rl.add_history_entry(line.as_ref());
        let tokens = line.split(" ");
        let tokens: Vec<&str> = tokens.collect();
        let command = tokens[0].trim();
        let home = dirs::home_dir().unwrap();

        if command == "cd" {
            let directory;

            if tokens.len() == 1 {
                directory = home.to_str().unwrap();
            }
            else {
                if tokens[1] == "~" {
                    directory = home.to_str().unwrap();
                } else {
                    directory = tokens[1];
                }
            }

            match env::set_current_dir(directory) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("{}: {}", directory, err);
                }
            }

            continue;
        }
        else if command == "exit" {
            break;
        }
        else if command == "" {
            continue;
        }

        let result = Command::new(command)
            .args(tokens[1..tokens.len()].into_iter())
            .output();

        match result {
            Ok(result) => {
                stdout().write_all(&result.stdout).unwrap();
                stderr().write_all(&result.stderr).unwrap();
            },
            Err(err) => {
                eprintln!("{}: {}", command, err);
            }
        }
    }

    rl.save_history(history).unwrap();
}
