use prettytable::{Cell, Row, Table as PTable};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::command_parser;
use command_parser::extract_info_from_create_table_cmd;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Column<T> {
    pub name: String,
    pub datatype: DataType,
    pub data: Vec<T>,
}

impl<T> Column<T> {
    pub fn new(name: String, datatype: String, data: Vec<T>) -> Column<T> {
        Column {
            name: name,
            datatype: DataType::new(datatype),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ColumnType {
    I32(Column<i32>),
    I64(Column<i64>),
    Float(Column<f32>),
    Double(Column<f64>),
    ColumnString(Column<String>),
    Bool(Column<bool>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub columns: Vec<ColumnType>,
    pub name: String,
    pub rows: Vec<Vec<String>>,
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

        let mut table_cols: Vec<ColumnType> = vec![];
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                match datatype {
                    "int" => table_cols.push(ColumnType::I32(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    "int64" => table_cols.push(ColumnType::I64(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    "float" => table_cols.push(ColumnType::Float(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    "double" => table_cols.push(ColumnType::Double(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    "string" => table_cols.push(ColumnType::ColumnString(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    "bool" => table_cols.push(ColumnType::Bool(Column::new(
                        name.to_string(),
                        datatype.to_string(),
                        vec![],
                    ))),
                    _ => panic!("Invalid datatype, {} ", datatype),
                }
            };
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
            rows: vec![],
        }
    }

    pub fn insert_row(&mut self, cols: Vec<String>, values: Vec<String>) {
        let mut sorted_values: Vec<String> = vec![];

        for column in &self.columns {
            match column {
                ColumnType::I32(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
                ColumnType::I64(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
                ColumnType::Float(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
                ColumnType::Double(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
                ColumnType::ColumnString(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
                ColumnType::Bool(col) => {
                    let idx = cols.iter().position(|c| c.to_string() == col.name).unwrap();
                    sorted_values.push(values[idx].to_string());
                }
            }
        }

        self.rows.push(sorted_values);
    }

    pub fn print_table(&self) {
        let mut table = PTable::new();
        table.add_row(row!["Column Name", "Data Type"]);

        for column in &self.columns {
            match column {
                ColumnType::I32(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
                ColumnType::I64(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
                ColumnType::Float(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
                ColumnType::Double(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
                ColumnType::ColumnString(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
                ColumnType::Bool(col) => {
                    table.add_row(row![col.name, col.datatype]);
                }
            }
        }

        table.printstd();
    }

    pub fn print_table_data(&self) {
        let mut table = PTable::new();
        let column_names = self
            .columns
            .iter()
            .map(|col| match col {
                ColumnType::I32(col) => Cell::new(&col.name),
                ColumnType::I64(col) => Cell::new(&col.name),
                ColumnType::Float(col) => Cell::new(&col.name),
                ColumnType::Double(col) => Cell::new(&col.name),
                ColumnType::ColumnString(col) => Cell::new(&col.name),
                ColumnType::Bool(col) => Cell::new(&col.name),
            })
            .collect::<Vec<Cell>>();

        table.add_row(Row::new(column_names));
        for row in &self.rows {
            let trow = row.iter().map(|r| Cell::new(r)).collect::<Vec<Cell>>();
            table.add_row(Row::new(trow));
        }

        table.printstd();
    }

    pub fn column_exist(&self, column: String) -> bool {
        self.columns.iter().any(|col| match col {
            ColumnType::I32(col) => col.name == column,
            ColumnType::I64(col) => col.name == column,
            ColumnType::Float(col) => col.name == column,
            ColumnType::Double(col) => col.name == column,
            ColumnType::ColumnString(col) => col.name == column,
            ColumnType::Bool(col) => col.name == column,
        })
    }

    pub fn does_column_value_match(&self, _column: String, _value: String) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_parser::extract_info_from_insert_cmd;

    #[test]
    fn tests_creating_a_table() {
        let command = String::from("CREATE TABLE users (id int, name string, bounty float)");
        let table = Table::new(command);

        let expected_columns = vec![
            ColumnType::I32(Column::new("id".to_string(), "int".to_string(), vec![])),
            ColumnType::ColumnString(Column::new(
                "name".to_string(),
                "string".to_string(),
                vec![],
            )),
            ColumnType::Float(Column::new(
                "bounty".to_string(),
                "float".to_string(),
                vec![],
            )),
        ];
        assert_eq!(table.name, "users");
        assert_eq!(table.columns, expected_columns);
    }

    // #[bench]
    // fn benches_insert(b: &mut test::Bencher) {
    //     let command =
    //         String::from("CREATE TABLE users (id int, name string, phone_number string, address string, gender string)");
    //     let mut table = Table::new(command);

    //     b.iter(|| {

    //         for i in 1..2 {
    //             let x = format!("INSERT INTO users (id, name, phone_number, address, gender) values ({}, 'tahmid', '01770169762', 'House 32, Road 1, Blcok C Banasree', 'male');", i);
    //             let (_table_name, columns, values) = extract_info_from_insert_cmd(x);
    //             table.insert_row(columns, values);
    //         }
    //     });
    //     // table.print_table_data();
    // }
}
