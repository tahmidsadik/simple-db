#[macro_use]
use prettytable::{Cell, Row, Table as PTable};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::result::Result;

use crate::command_parser;
use crate::parser::select::SelectQuery;
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
            _ => {
                println!("Invalid data type given {}", cmd);
                return DataType::Invalid;
            }
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
    pub is_indexed: bool,
    pub index: BTreeMap<String, usize>,
    pub is_primary_key: bool,
}

impl ColumnHeader {
    pub fn new(name: String, datatype: String, is_primary_key: bool) -> ColumnHeader {
        ColumnHeader {
            name: name,
            datatype: DataType::new(datatype),
            is_indexed: if is_primary_key { true } else { false },
            index: BTreeMap::new(),
            is_primary_key,
        }
    }

    pub fn get_mut_index(&mut self) -> &mut BTreeMap<String, usize> {
        return &mut self.index;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ColumnData {
    Int(Vec<i32>),
    Str(Vec<String>),
    Float(Vec<f32>),
    Bool(Vec<bool>),
    None,
}

impl ColumnData {
    fn get_serialized_col_data(&self) -> Vec<String> {
        match self {
            ColumnData::Int(cd) => cd.iter().map(|v| v.to_string()).collect(),
            ColumnData::Float(cd) => cd.iter().map(|v| v.to_string()).collect(),
            ColumnData::Str(cd) => cd.iter().map(|v| v.to_string()).collect(),
            ColumnData::Bool(cd) => cd.iter().map(|v| v.to_string()).collect(),
            ColumnData::None => panic!("Found None in columns"),
        }
    }

    fn count(&self) -> usize {
        match self {
            ColumnData::Int(cd) => cd.len(),
            ColumnData::Float(cd) => cd.len(),
            ColumnData::Str(cd) => cd.len(),
            ColumnData::Bool(cd) => cd.len(),
            ColumnData::None => panic!("Found None in columns"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub columns: Vec<ColumnHeader>,
    pub name: String,
    pub rows: HashMap<String, ColumnData>,
}

impl Table {
    pub fn new(cmd: String) -> Table {
        let (table_name, columns) = extract_info_from_create_table_cmd(cmd);

        let mut table_cols: Vec<ColumnHeader> = vec![];
        let mut table_data: HashMap<String, ColumnData> = HashMap::new();
        for c in &columns {
            table_cols.push(ColumnHeader::new(
                c.name.to_string(),
                c.datatype.to_string(),
                c.is_pk,
            ));

            match DataType::new(c.datatype.to_string()) {
                DataType::Int => table_data.insert(c.name.to_string(), ColumnData::Int(vec![])),
                DataType::Float => table_data.insert(c.name.to_string(), ColumnData::Float(vec![])),
                DataType::Str => table_data.insert(c.name.to_string(), ColumnData::Str(vec![])),
                DataType::Bool => table_data.insert(c.name.to_string(), ColumnData::Bool(vec![])),
                DataType::Invalid => table_data.insert(c.name.to_string(), ColumnData::None),
            };
        }

        Table {
            columns: table_cols,
            name: table_name.to_string(),
            rows: table_data,
        }
    }

    pub fn does_violate_unique_constraint(
        &self,
        cols: &Vec<String>,
        values: &Vec<String>,
    ) -> Result<(), String> {
        for c in &self.columns {
            if c.is_primary_key {
                let index = &c.index;
                for (idx, name) in cols.iter().enumerate() {
                    if *name == c.name {
                        let val = &values[idx];
                        match index.contains_key(val) {
                            true => {
                                return Err(format!(
                                    "Error: unique constraint violation for column {}.
                            Value {} already exists for column {}",
                                    *name, val, *name
                                ));
                            }
                            false => return Ok(()),
                        };
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn insert_row(&mut self, cols: &Vec<String>, values: &Vec<Vec<String>>) {
        for i in 0..cols.len() {
            let key = &cols[i];
            let table_col_data = self.rows.get_mut(&key.to_string()).unwrap();

            let mut column_headers = self
                .columns
                .iter_mut()
                .filter(|c| c.name == key.to_string())
                .collect::<Vec<&mut ColumnHeader>>();

            let index = column_headers
                .first_mut()
                .expect("Couldn't find column to insert row")
                .get_mut_index();

            for value in values {
                // TODO: only check for unique constraint violation if unique constraint exists for
                // the column
                let val = &value[i];
                match table_col_data {
                    ColumnData::Int(c_vec) => c_vec.push(val.parse::<i32>().unwrap()),
                    ColumnData::Float(c_vec) => c_vec.push(val.parse::<f32>().unwrap()),
                    ColumnData::Bool(c_vec) => c_vec.push(val.parse::<bool>().unwrap()),
                    ColumnData::Str(c_vec) => c_vec.push(val.to_string()),
                    ColumnData::None => panic!("None data Found"),
                }
                index.insert(val.to_string(), table_col_data.count() - 1);
            }
        }
    }

    pub fn execute_select_query(&self, sq: SelectQuery) {
        let mut data: Vec<Vec<String>> = vec![];
        for col in &sq.projection {
            let row = self.rows.get(col).unwrap();
            let column = row.get_serialized_col_data();
            data.push(column);
        }
        let rotated_data = util::rotate_2d_vec(&data);
        util::pretty_print(&rotated_data, sq.projection);
    }

    pub fn print_table(&self) {
        let mut table = PTable::new();
        table.add_row(row!["Column Name", "Data Type"]);

        for col in &self.columns {
            table.add_row(row![col.name, col.datatype]);
        }

        println!("TableName: {}", self.name);
        table.printstd();
        println!();
    }

    pub fn print_table_data(&self) {
        let mut p_table = PTable::new();

        let cnames = self
            .columns
            .iter()
            .map(|col| col.name.to_string())
            .collect::<Vec<String>>();

        let header_row = Row::new(
            cnames
                .iter()
                .map(|col| Cell::new(&col))
                .collect::<Vec<Cell>>(),
        );

        let first_col_data = self.rows.get(&self.columns.first().unwrap().name).unwrap();

        let num_rows = match first_col_data {
            ColumnData::Int(cd) => cd.len(),
            ColumnData::Float(cd) => cd.len(),
            ColumnData::Bool(cd) => cd.len(),
            ColumnData::Str(cd) => cd.len(),
            ColumnData::None => panic!("Found None data"),
        };

        let mut print_table_rows: Vec<Row> = vec![Row::new(vec![]); num_rows];

        for col_name in &cnames {
            let col_val = self
                .rows
                .get(col_name)
                .expect("Can't find any rows with the given column");
            let columns: Vec<String> = col_val.get_serialized_col_data();

            for i in 0..num_rows {
                print_table_rows[i].add_cell(Cell::new(&columns[i]));
            }
        }

        p_table.add_row(header_row);
        for row in print_table_rows {
            p_table.add_row(row);
        }

        p_table.printstd();
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

        let expected_column_names = vec![
            "id".to_string(),
            "name".to_string(),
            "bounty".to_string(),
            "unknown".to_string(),
        ];
        let expected_column_types = vec![
            "Int".to_string(),
            "Str".to_string(),
            "Float".to_string(),
            "Invalid".to_string(),
        ];

        let column_names = table
            .columns
            .iter()
            .map(|c| c.name.to_string())
            .collect::<Vec<String>>();

        let column_types = table
            .columns
            .iter()
            .map(|c| c.datatype.to_string())
            .collect::<Vec<String>>();

        assert_eq!(table.name, "users");
        assert_eq!(column_names, expected_column_names);
        assert_eq!(column_types, expected_column_types);
    }

    #[test]
    fn tests_unique_constraint_violation_on_primary_key() {
        let command = String::from("CREATE TABLE users (id int PRIMARY KEY, name string)");
        let mut table = Table::new(command);
        let cols = vec!["id".to_string(), "name".to_string()];
        let val = vec!["1".to_string(), "tahmid".to_string()];
        table.does_violate_unique_constraint(&cols, &val);
        table.insert_row(&cols, &vec![val.clone()]);
        assert_eq!(
            table.does_violate_unique_constraint(&cols, &val).is_err(),
            true
        );
    }
}

mod util {
    use super::{Cell, PTable, Row};

    pub fn rotate_2d_vec(data: &Vec<Vec<String>>) -> Vec<Vec<&String>> {
        match data.first() {
            None => vec![vec![]],
            _ => {
                let number_of_rows = data.first().unwrap().len();
                let number_of_cols = data.len();
                let mut ret_data: Vec<Vec<&String>> = vec![vec![]; number_of_rows];

                println!("start rotating");
                for row_idx in 0..number_of_rows {
                    for col_idx in 0..number_of_cols {
                        println!("row idx = {}, col idx = {}", row_idx, col_idx);
                        ret_data[row_idx].push(&data[col_idx][row_idx]);
                    }
                }

                return ret_data;
            }
        }
    }

    pub fn pretty_print(data: &Vec<Vec<&String>>, header: Vec<String>) {
        let mut p_table = PTable::new();

        for d in data {
            for i in d {
                println!("i = {}", i);
            }
        }

        p_table.add_row(Row::new(
            header.iter().map(|h| Cell::new(h)).collect::<Vec<Cell>>(),
        ));

        for row in data {
            p_table.add_row(Row::new(
                row.iter().map(|c| Cell::new(*c)).collect::<Vec<Cell>>(),
            ));
        }
        p_table.printstd();
    }
}
