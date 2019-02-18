use std::io::*;
use std::process::Command;

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
                let mut tokens = line.split(" ");
                let vec: Vec<&str> = tokens.collect();
                let output = Command::new(vec[0])
                    .arg(vec[1..vec.len()].join(" "))
                    .output()
                    .expect("failed to execute process");
                stdout().write_all(&output.stdout).unwrap();
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
