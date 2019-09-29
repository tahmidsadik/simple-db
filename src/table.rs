use prettytable::{Cell, Row, Table as PTable};

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

pub enum RowValue {
    Int(i32),
    Str(String),
    Float(f32),
}

impl RowValue {
    pub fn new(valtype: DataType, val: String) -> RowValue {
        match valtype {
            DataType::Int => RowValue::Int(val.parse::<i32>().unwrap()),
            DataType::Str => RowValue::Str(val),
            DataType::Float => RowValue::Float(val.parse::<f32>().unwrap()),
            DataType::Invalid => RowValue::Str(val),
        }
    }
}

pub struct Table {
    pub columns: Vec<Column>,
    pub name: String,
    pub rows: HashMap<String, Vec<String>>,
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
        let mut table_rows = HashMap::new();
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(Column::new(name.to_string(), datatype.to_string()));
                table_rows.insert(name.to_string(), vec![]);
                // match DataType::new(datatype.to_string()) {
                //     DataType::Int => {
                //         table_rows.insert(name.to_string(), vec![]);
                //     }
                //     DataType::Str => {
                //
                //     }
                //     DataType::Float => {
                //
                //     }
                //     DataType::Double => {
                //
                //     }
                //     DataType::Invalid => {
                //
                //     }
                //
                // };
            };
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
            rows: table_rows,
        }
    }

    pub fn insert_row(&mut self, rows: HashMap<String, String>) {
        for (k, v) in rows {
            println!("key = {}, val ={}", k, v);
            let val = self.rows.get_mut(&k).unwrap();
            val.push(v);
        }

        for (k, v) in &self.rows {
            for i in v {
                println!("{}", i);
            }
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

    pub fn print_table_data(&self) {
        let mut table = PTable::new();
        let column_names = self
            .columns
            .iter()
            .map(|col| Cell::new(&col.name))
            .collect::<Vec<Cell>>();

        let cnames = self
            .columns
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>();

        let num_rows = self
            .rows
            .get(&self.columns.first().unwrap().name)
            .unwrap()
            .len();
        table.add_row(Row::new(column_names));

        for i in 0..num_rows {
            let mut row: Vec<Cell> = vec![];
            for cname in &cnames {
                let v = self.rows.get(cname).unwrap();
                row.push(Cell::new(&v[i]));
            }
            table.add_row(Row::new(row));
        }
        table.printstd();
    }

    pub fn column_exist(&self, column: String) -> bool {
        self.columns.iter().any(|col| col.name == column)
    }

    pub fn does_column_value_match(&self, _column: String, _value: String) {}
}
