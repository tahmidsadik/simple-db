use std::env;
use std::io::{stdin, stdout, Write};

mod database;
mod table;

use database::Database;
use table::Table;

enum MetaCommand {
    Exit,
    ListTables,
    Unknown(String),
}

impl MetaCommand {
    fn new(command: String) -> MetaCommand {
        match command.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".tables" => MetaCommand::ListTables,
            _ => MetaCommand::Unknown(command),
        }
    }
}

enum DbCommand {
    Insert(String),
    Delete(String),
    Update(String),
    CreateTable(String),
    Unknown(String),
}

impl DbCommand {
    fn new(command: String) -> DbCommand {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "insert" => DbCommand::Insert(command),
            "update" => DbCommand::Update(command),
            "delete" => DbCommand::Delete(command),
            "create" => DbCommand::CreateTable(command),
            _ => DbCommand::Unknown(command),
        }
    }

    fn insert(command: String) {
        let tokens = command.split(" ").skip(1).collect::<Vec<&str>>();
        if let [id, username, email] = tokens[..] {
            println!(
                "id = {}, username = {}, and email = {}",
                id, username, email
            );
        } else {
            println!("Invalid argument passed for insert statement");
        }
    }
}

enum CommandType {
    MetaCommand(MetaCommand),
    DbCommand(DbCommand),
}

fn handle_commands(cmd: &String) -> CommandType {
    match cmd.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(cmd.to_owned())),
        false => CommandType::DbCommand(DbCommand::new(cmd.to_owned())),
    }
}

fn handle_meta_command(cmd: MetaCommand, db: &Database) {
    match cmd {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::ListTables => {
            for table in &db.tables {
                for column in &table.columns {
                    println!("Column Name {}, Columns Type {}", column.name, column.datatype);
                }
            }
        },
        MetaCommand::Unknown(cmd) => println!("Unrecognized meta command {}", cmd),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command = String::new();
    for arg in args {
        println!("{}", arg);
    }

    let mut db = Database::new();

    loop {
        print!("sqlite> ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");

        match handle_commands(&command.trim().to_owned()) {
            CommandType::DbCommand(cmd) => match cmd {
                DbCommand::Insert(ccmd) => {
                    println!("Insert Command {}", ccmd);
                    let tokens = ccmd.split(" ").skip(2).collect::<Vec<&str>>();
                    let name = *tokens.first().unwrap();
                    match db.table_exists(name.to_string()) {
                        true => println!("Table exists"),
                        false => println!("Table doesn't exist"),
                    }
                    DbCommand::insert(ccmd);
                }
                DbCommand::Update(ccmd) => println!("Update Command {}", ccmd),
                DbCommand::Delete(ccmd) => println!("Delete Command {}", ccmd),
                DbCommand::CreateTable(ccmd) => {
                    println!("Acknowledged create table command {}", ccmd);
                    db.tables.push(Table::new(ccmd));
                    for table in &db.tables {
                        for col in &table.columns {
                            println!(
                                "Column name = {}, Column Datatype = {}",
                                col.name, col.datatype
                            );
                        }
                    }
                }
                DbCommand::Unknown(ccmd) => println!("Unknown Command {}", ccmd),
            },
            CommandType::MetaCommand(cmd) => {
                handle_meta_command(cmd, &db);
            }
        }
        command = "".to_string();
    }
}
