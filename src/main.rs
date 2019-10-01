#[macro_use]
extern crate prettytable;
extern crate regex;

pub mod command_parser;
mod database;
pub mod table;

use std::env;
use std::fs::File;
use std::io::prelude::Write;
use std::io::Read;
use std::io::{stdin, stdout};

use command_parser::extract_info_from_insert_cmd;
use database::Database;
use table::Table;

enum MetaCommand {
    Exit,
    ListTables,
    PrintData,
    Persist,
    Restore,
    Unknown(String),
}

impl MetaCommand {
    fn new(command: String) -> MetaCommand {
        match command.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".tables" => MetaCommand::ListTables,
            ".data" => MetaCommand::PrintData,
            ".persist" => MetaCommand::Persist,
            ".restore" => MetaCommand::Restore,
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

fn handle_meta_command(cmd: MetaCommand, db: &mut Database) {
    match cmd {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::ListTables => {
            for table in &db.tables {
                table.print_table();
            }
        }
        MetaCommand::PrintData => {
            for table in &db.tables {
                table.print_table_data();
            }
        }
        MetaCommand::Persist => {
            let encoded = bincode::serialize(&db).unwrap();
            let mut file = File::create("dbfile1").unwrap();
            file.write_all(&encoded);
        }
        MetaCommand::Restore => {
            let mut file = File::open("dbfile1").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("{}", contents);
            let encoded = contents.as_bytes();
            println!("{:?}", encoded);
            let mut decoded_db: Database = bincode::deserialize(&encoded[..]).unwrap();
            println!("db tables length = {}", db.tables.len());
            // db.tables[0].print_table();
            // db.tables[0].print_table_data();
            // db = &decoded_db;
        }
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
                    let (table, columns, values) = extract_info_from_insert_cmd(ccmd.to_owned());

                    // TODO: we need to find the correct table not just take the first one.

                    match db.table_exists(table.to_string()) {
                        true => {
                            println!("Table exists");

                            let tt = db.tables.first().unwrap();

                            match columns.iter().all(|c| tt.column_exist(c.to_string())) {
                                true => {
                                    println!("all columns exist");
                                    println!("let's insert");
                                    db.tables.first_mut().unwrap().insert_row(columns, values);
                                }
                                false => {
                                    println!("Cannot insert, some of the columns do not exist");
                                }
                            }
                        }
                        false => println!("Table doesn't exist"),
                    }
                }

                DbCommand::Update(ccmd) => println!("Update Command {}", ccmd),
                DbCommand::Delete(ccmd) => println!("Delete Command {}", ccmd),
                DbCommand::CreateTable(ccmd) => {
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
                handle_meta_command(cmd, &mut db);
            }
        }
        command = "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_extracting_info_from_insert_cmd() {
        let input = String::from("insert into users (id, name, age) values(1, hello, 27);");
        let (table, columns, values) = extract_info_from_insert_cmd(input);
        assert_eq!(table, "users");
        assert_eq!(
            columns,
            vec!["id".to_string(), "name".to_string(), "age".to_string()]
        );
        assert_eq!(
            values,
            vec!["1".to_string(), "hello".to_string(), "27".to_string()]
        );
    }
}
