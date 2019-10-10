use prettytable::{Cell, Row, Table as PTable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::command_parser;
use command_parser::extract_info_from_create_table_cmd;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DataType {
    Int,
    Str,
    Float,
    Bool,
    Invalid,
}

impl DataType {
    pub fn new(cmd: String) -> DataType {
        match cmd.to_lowercase().as_ref() {
            "int" => DataType::Int,
            "string" => DataType::Str,
            "float" => DataType::Float,
            "double" => DataType::Float,
            "bool" => DataType::Bool,
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
            DataType::Bool => f.write_str("Boolean"),
            DataType::Invalid => f.write_str("Invalid"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ColumnHeader {
    pub name: String,
    pub datatype: DataType,
}

impl ColumnHeader {
    pub fn new(name: String, datatype: String) -> ColumnHeader {
        ColumnHeader {
            name: name,
            datatype: DataType::new(datatype),
        }
    }
}

// impl FromIterator for ColumnHeader {
//     fn from_iter<I: IntoIterator<Item=ColumnHeader>>(iter: I) -> Self {
//         let mut c = ColumnHeader::new(iter.);

//         for i in iter {
//             c.add(i);
//         }

//         c
//     }
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ColumnData {
    Int(Vec<i32>),
    Str(Vec<String>),
    Float(Vec<f32>),
    Bool(Vec<bool>),
    None,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub columns: Vec<ColumnHeader>,
    pub name: String,
    pub rows: HashMap<String, ColumnData>,
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

        let mut table_cols: Vec<ColumnHeader> = vec![];
        let mut table_data: HashMap<String, ColumnData> = HashMap::new();
        for s in columns {
            if let [name, datatype] = s.trim().split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(ColumnHeader::new(name.to_string(), datatype.to_string()));

                match DataType::new(datatype.to_string()) {
                    DataType::Int => table_data.insert(name.to_string(), ColumnData::Int(vec![])),
                    DataType::Float => {
                        table_data.insert(name.to_string(), ColumnData::Float(vec![]))
                    }
                    DataType::Str => table_data.insert(name.to_string(), ColumnData::Str(vec![])),
                    DataType::Bool => table_data.insert(name.to_string(), ColumnData::Bool(vec![])),
                    DataType::Invalid => table_data.insert(name.to_string(), ColumnData::None),
                };
            }
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
            rows: table_data,
        }
    }

    // pub fn get_column_datatype(&self, col_name: String) -> DataType {
    //     let column = self
    //         .columns
    //         .iter()
    //         .filter(|&c| c.name == col_name)
    //         .collect::<Vec<&ColumnHeader>>()
    //         .first()
    //         .unwrap();
    //     column.datatype
    // }

    pub fn insert_row(&mut self, cols: Vec<String>, values: Vec<String>) {
        for i in 0..cols.len() {
            let key = &cols[i];
            let val = &values[i];
            let mut table_col_data = self.rows.get_mut(&key.to_string()).unwrap();
            match table_col_data {
                ColumnData::Int(c_vec) => c_vec.push(val.parse::<i32>().unwrap()),
                ColumnData::Float(c_vec) => c_vec.push(val.parse::<f32>().unwrap()),
                ColumnData::Bool(c_vec) => c_vec.push(val.parse::<bool>().unwrap()),
                ColumnData::Str(c_vec) => c_vec.push(val.to_string()),
                ColumnData::None => panic!("None data Found"),
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

        let first_col_data = self.rows.get(&self.columns.first().unwrap().name).unwrap();

        let num_rows = match first_col_data {
            ColumnData::Int(cd) => cd.len(),
            ColumnData::Float(cd) => cd.len(),
            ColumnData::Bool(cd) => cd.len(),
            ColumnData::Str(cd) => cd.len(),
            ColumnData::None => panic!("Found None data"),
        };

        table.add_row(Row::new(column_names));

        for i in 0..num_rows {
            let mut row: Vec<Cell> = vec![];
            for cname in &cnames {
                let v = self.rows.get(cname).unwrap();

                match v {
                    ColumnData::Int(cd) => row.push(Cell::new(&cd[i].to_string())),
                    ColumnData::Float(cd) => row.push(Cell::new(&cd[i].to_string())),
                    ColumnData::Bool(cd) => row.push(Cell::new(&cd[i].to_string())),
                    ColumnData::Str(cd) => row.push(Cell::new(&cd[i].to_string())),
                    ColumnData::None => panic!("Found None data"),
                }
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
            ColumnHeader::new("id".to_string(), "int".to_string()),
            ColumnHeader::new("name".to_string(), "string".to_string()),
            ColumnHeader::new("bounty".to_string(), "float".to_string()),
            ColumnHeader::new("unknown".to_string(), "unknown".to_string()),
        ];
        assert_eq!(table.name, "users");
        assert_eq!(table.columns, expected_columns);
    }

    #[bench]
    fn benches_insert(b: &mut test::Bencher) {
        let command =
            String::from("CREATE TABLE users (id int, name string, phone_number string, address string, gender string)");
        let mut table = Table::new(command);

        b.iter(|| {
            let x = format!("INSERT INTO users (id, name, phone_number, address, gender) values (1, 'tahmid', '01770169762', 'House 32, Road 1, Blcok C Banasree', 'male');");
            let (_table_name, columns, values) = extract_info_from_insert_cmd(x);
            table.insert_row(columns, values);
        });
        table.print_table_data();
    }
}
