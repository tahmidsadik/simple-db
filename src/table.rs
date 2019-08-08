use prettytable::{Table as PTable, Row as PTRow, Cell as PTCell};
extern crate regex;

use regex::Regex;
use std::string::String;
use std::fmt;

pub enum DataType {
    Int,
    Str,
    Float,
    Invalid,
}

impl DataType {
    fn new(cmd: String) -> DataType {
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
            DataType::Invalid => f.write_str("Invalid")
        }
    }
}

pub struct Column {
    pub name: String,
    pub datatype: DataType,
}

impl Column {
    fn new(name: String, datatype: String) -> Column {
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
        let tokens = cmd.split(" ").skip(2).collect::<Vec<&str>>();
        let table_name = tokens.first().expect("No table name given");

        let columns_matcher = Regex::new(r"\((.|\n)*\)").unwrap();
        let reg_matcher_obj = columns_matcher.find(cmd.as_ref()).unwrap();
        let columns: Vec<String> = cmd[(reg_matcher_obj.start() + 1)..(reg_matcher_obj.end() - 1)]
            .trim()
            .split(",")
            .map(|x| x.clone().replace("\n", ""))
            .collect();

        let mut table_cols: Vec<Column> = vec![];
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(Column {
                    name: name.to_string(),
                    datatype: DataType::new(datatype.to_string()),
                });
            };
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
        }
    }

    pub fn printTable(&self) {
        let mut table = PTable::new();
        table.add_row(row!["Column Name", "Data Type"]);

        for col in &self.columns {
            table.add_row(row![col.name, col.datatype]);
        }

        table.printstd();
    }
}
