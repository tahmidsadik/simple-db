use prettytable::Table as PTable;

use crate::command_parser::{extract_info_from_create_table_cmd, extract_info_from_insert_cmd};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum DataType {
    Int,
    Str,
    Float,
    Invalid,
}

impl DataType {
    pub fn new(cmd: String) -> DataType {
        match cmd.to_lowercase().as_ref() {
            "int" => DataType::Int,
            "string" => DataType::Str,
            "float" => DataType::Float,
            "double" => DataType::Float,
            _ => DataType::Invalid,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::Int => f.write_str("Int"),
            DataType::Str => f.write_str("Str"),
            DataType::Float => f.write_str("Float"),
            DataType::Invalid => f.write_str("Invalid"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}

impl Column {
    pub fn new(name: String, datatype: String) -> Column {
        Column {
            name: name,
            datatype: DataType::new(datatype),
        }
    }
}

pub struct Table {
    pub columns: Vec<Column>,
    pub name: String,
}

impl Table {
    pub fn new(cmd: String) -> Table {
        let hm = extract_info_from_create_table_cmd(cmd);

        let table_name = hm
            .get("tname")
            .expect("Error while trying to parse table name from insert command");
        let columns = hm
            .get("columns")
            .expect("Error while trying to parse table name from insert command");

        let columns: Vec<&str> = columns.split(",").collect();

        let mut table_cols: Vec<Column> = vec![];
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(Column::new(name.to_string(), datatype.to_string()));
            };
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
        }
    }

    pub fn print_table(&self) {
        let mut table = PTable::new();
        table.add_row(row!["Column Name", "Data Type"]);

        for col in &self.columns {
            table.add_row(row![col.name, col.datatype]);
        }

        table.printstd();
    }

    pub fn column_exist(&self, column: String) -> bool {
        self.columns.iter().any(|col| col.name == column)
    }

    pub fn does_column_value_match(&self, _column: String, _value: String) {}
}
