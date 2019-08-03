use std::env;
use std::io::{stdin, stdout, Write};

enum MetaCommand {
    Exit,
    Unknown(String)
}

impl MetaCommand {
    fn new(command: String) -> MetaCommand {
        match command.as_ref() {
            ".exit" => MetaCommand::Exit,
            _ => MetaCommand::Unknown(command)
        }
    }
}

enum DbCommand {
    Insert(String),
    Delete(String),
    Update(String),
    Unknown(String)
}

impl DbCommand {
    fn new(command: String) -> DbCommand {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "insert" => DbCommand::Insert(command),
            "update" => DbCommand::Update(command),
            "delete" => DbCommand::Delete(command),
            _ => DbCommand::Unknown(command),
            
        }
    }
}

enum CommandType {
    MetaCommand(MetaCommand),
    DbCommand(DbCommand)
}

fn handle_commands(cmd: &String) -> CommandType {
    match cmd.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(cmd.to_owned())),
        false => CommandType::DbCommand(DbCommand::new(cmd.to_owned()))
    }
}

fn handle_meta_command(cmd: MetaCommand) {
    match cmd {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::Unknown(cmd) => println!("Unrecognized meta command {}", cmd)
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
            CommandType::DbCommand(cmd) => {
                match cmd {
                    DbCommand::Insert(ccmd) => println!("Insert Command {}", ccmd),
                    DbCommand::Update(ccmd) => println!("Update Command {}", ccmd),
                    DbCommand::Delete(ccmd) => println!("Delete Command {}", ccmd),
                    DbCommand::Unknown(ccmd) => println!("Unknown Command {}", ccmd),
                }
            }
            CommandType::MetaCommand(cmd) => {
                handle_meta_command(cmd);
            }
        }
        command = "".to_string();
    }
}
