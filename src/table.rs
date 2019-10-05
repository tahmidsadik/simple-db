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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub columns: Vec<Column>,
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

        let mut table_cols: Vec<Column> = vec![];
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(Column::new(name.to_string(), datatype.to_string()));
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
            let idx = cols
                .iter()
                .position(|c| c.to_string() == column.name)
                .unwrap();
            sorted_values.push(values[idx].to_string());
        }

        self.rows.push(sorted_values);
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

        table.add_row(Row::new(column_names));
        for row in &self.rows {
            let trow = row.iter().map(|r| Cell::new(r)).collect::<Vec<Cell>>();
            table.add_row(Row::new(trow));
        }

        table.printstd();
    }

    pub fn column_exist(&self, column: String) -> bool {
        self.columns.iter().any(|col| col.name == column)
    }

    pub fn does_column_value_match(&self, _column: String, _value: String) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_parser::extract_info_from_insert_cmd;

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
