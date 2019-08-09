#[macro_use]
extern crate prettytable;
extern crate regex;

mod command_parser;
mod database;
mod table;

use std::env;
use std::io::{stdin, stdout, Write};

use command_parser::extract_info_from_insert_cmd;
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
                table.print_table();
            }
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
                    println!("Insert Command {}", ccmd);
                    println!("Start capturing");
                    extract_info_from_insert_cmd(ccmd.to_owned());

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

#[cfg(test)]
mod tests {
    use super::*;
    use command_parser::{extract_info_from_create_table_cmd, sanitize_user_input};
    use table::{Column, DataType};

    #[test]
    fn sanitize_input_trims_single_whitespaces_correctly_from_start_and_end() {
        let input = String::from(" hello ");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_trims_multiple_whitespaces_correctly_from_start_and_end() {
        let input = String::from("         hello         ");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_lowercases_the_input() {
        let input = String::from("HELLO WORLD GoodBye World");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello world goodbye world");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_replaces_multiple_whitespaces_into_single_one_inside_string() {
        // should turn "hello     world       end" => "hello world end"
        let input = String::from("hello       world        end");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello world end");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn parses_correctly_from_create_table_cmd() {
        let input = String::from("CREATE TABLE users (id int, name string)");
        let parsed_cmd_hm = extract_info_from_create_table_cmd(input);
        let table_name = String::from(parsed_cmd_hm.get("tname").unwrap());
        let columns_schema = String::from(parsed_cmd_hm.get("columns").unwrap());

        let expected_table_name = String::from("users");
        let expected_columns_schema = String::from("id int, name string");
        assert_eq!(table_name, expected_table_name);
        assert_eq!(columns_schema, expected_columns_schema);
    }

    #[test]
    fn tests_creating_a_table() {
        let command =
            String::from("CREATE TABLE users (id int, name string, bounty float, unknown unknown)");
        let table = Table::new(command);

        let expected_columns = vec![
            Column::new("id".to_string(), "int".to_string()),
            Column::new("name".to_string(), "string".to_string()),
            Column::new("bounty".to_string(), "float".to_string()),
            Column::new("unknown".to_string(), "unknown".to_string()),
        ];
        assert_eq!(table.name, "users");
        assert_eq!(table.columns, expected_columns);
    }

    #[test]
    fn sanitize_input_trims_whitespaces_correctly() {
        let input = String::from(" hello ");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello");
        assert_eq!(sanitized_input, expected_output);
    }

}
