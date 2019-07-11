use std::env;
use std::io::{stdin, stdout, Write };

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command = String::new();
    for arg in args {
        println!("{}", arg);
    }

    loop {
        print!("sqlite> ");
        stdout().flush();

        stdin().read_line(&mut command).expect("Error while trying to read from stdin");
        if command.trim() == ".exit" {
            break;
        }
        else {
            println!("Unrecgnized command!")
        }
        command = "".to_string();
    }
}
