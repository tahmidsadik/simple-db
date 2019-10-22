//#![feature(test)]
#[macro_use]
extern crate prettytable;
extern crate regex;
//extern crate test;

use std::env;
use std::fs::File;
use std::io::prelude::Write;
use std::io::BufWriter;
use std::io::{stdin, stdout};

use command_parser::extract_info_from_insert_cmd;
use database::Database;
use parser::select::SelectQuery;
use table::Table;

mod command_parser;
mod database;
mod parser;
mod table;

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
    Select(String),
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
            "select" => DbCommand::Select(command),
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
            if db.tables.len() == 0 {
                println!("No tables found");
            }
            for table in &db.tables {
;                table.print_table();
            }
        }
        MetaCommand::PrintData => {
            for table in &db.tables {
                table.print_table_data();
            }
        }
        MetaCommand::Persist => {
            let mut buffered_writer = BufWriter::new(File::create("dbfile1.bin").unwrap());
            bincode::serialize_into(&mut buffered_writer, &db)
                .expect("Error while trying to serialize to binary data");
        }
        MetaCommand::Restore => {
            let mut file = File::open("dbfile1.bin").unwrap();
            // let mut buffer = Vec::<u8>::new();
            // file.read_to_end(&mut buffer).unwrap();
            // let mut decoded_db: Database = bincode::deserialize(&buffer[..]).unwrap();
            let decoded_db: Database = bincode::deserialize_from(&mut file).unwrap();
            // db.tables[0].print_table();
            // db.tables[0].print_table_data();
            *db = decoded_db;
        }
        MetaCommand::Unknown(cmd) => println!("Unrecognized meta command {}", cmd),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command = String::new();
    let mut db = Database::new();

    // handle_meta_command(MetaCommand::Restore, &mut db);

    loop {
        print!("sdb> ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut command)
            .expect("Error while trying to read from stdin");
        match handle_commands(&command.trim().to_owned()) {
            CommandType::DbCommand(cmd) => match cmd {
                DbCommand::Select(ccmd) => {
                    let select_query = SelectQuery::new(&ccmd);

                    match select_query {
                        Ok(sq) => match db.table_exists((&sq.from).to_string()) {
                            true => {
                                let db_table = db.get_table((&sq.from).to_string());
                                for col in &sq.projection {
                                    if !db_table.column_exist((&col).to_string()) {
                                        println!("Cannot execute query, cannot find column {} in table {}",col, db_table.name);
                                        return;
                                    }
                                }

                                db_table.execute_select_query(sq);
                            }
                            false => {
                                println!(
                                    "Cannot execute query the table {} doesn't exists",
                                    sq.from
                                );
                            }
                        },
                        Err(error) => {
                            println!("{}", error);
                        }
                    }
                }
                DbCommand::Insert(ccmd) => {
                    let (table, columns, values) = extract_info_from_insert_cmd(ccmd.to_owned());
                    match db.table_exists(table.to_string()) {
                        true => {
                            let db_table = db.get_table_mut(table.to_string());
                            match columns.iter().all(|c| db_table.column_exist(c.to_string())) {
                                true => {
                                    for value in &values {
                                        match db_table
                                            .does_violate_unique_constraint(&columns, value)
                                        {
                                            Err(err) => {
                                                println!("Unique key constaint violation: {}", err)
                                            }
                                            Ok(()) => {
                                                db_table.insert_row(&columns, &values);
                                            }
                                        }
                                    }
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
