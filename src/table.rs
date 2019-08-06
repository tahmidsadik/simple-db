extern crate regex;
use regex::Regex;
use std::string::String;

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
}

impl Table {
    pub fn new(cmd: String) -> Table {
        println!("{}", cmd);
        let tokens = cmd.split(" ").skip(2).collect::<Vec<&str>>();
        let table_name = tokens.first().expect("No table name given");

        let columns_matcher = Regex::new(r"\((.|\n)*\)").unwrap();
        let columns: Vec<String> = columns_matcher
            .find(cmd.as_ref())
            .unwrap()
            .as_str()
            .trim()
            .split(",")
            .map(|x| x.clone().replace("\n", ""))
            .collect();

        let mut table_cols: Vec<Column> = vec![];
        for s in columns {
            if let [name, datatype] = s.split(" ").collect::<Vec<&str>>()[..] {
                table_cols.push(Column {
                    name: name.to_string(),
                    datatype: DataType::new(datatype.to_string()),
                });
            };
        }

        Table {
            columns: table_cols,
        }
    }
}
