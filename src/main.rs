use std::env;
use std::io::{stdin, stdout, Write};

enum MetaCommandResult {
    Success(String),
    Unknown,
}

fn handle_commands(cmd: &String) -> MetaCommandResult {
    match cmd.starts_with(".") {
        true => MetaCommandResult::Success(cmd.to_owned()),
        false => MetaCommandResult::Unknown,
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command = String::new();
    for arg in args {
        println!("{}", arg);
    }

    loop {
        print!("sqlite> ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");

        match handle_commands(&command.trim().to_owned()) {
            MetaCommandResult::Success(cmd) => {
                match cmd.as_ref() {
                    ".exit" => break,
                    _ => println!("Unrecognized meta command {}", cmd)
                }
            }
            MetaCommandResult::Unknown => println!("Unrecgnized command!"),
        }
        command = "".to_string();
    }
}
