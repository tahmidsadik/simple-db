use sqlparser::ast::{ColumnOption, DataType, ObjectName, Statement};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

#[derive(PartialEq, Debug)]
pub struct ParsedColumn {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub is_nullable: bool,
}

#[derive(Debug)]
pub struct CreateQuery {
    pub table_name: String,         // table name
    pub columns: Vec<ParsedColumn>, // columns that will be fetched
}

impl CreateQuery {
    pub fn new(statement: &Statement) -> Result<CreateQuery, String> {
        match statement {
            Statement::CreateTable {
                name,
                columns,
                constraints: constraints,
                with_options: _with_options,
                external: _external,
                file_format: _file_format,
                location: _location,
            } => {
                let table_name = name;
                let mut parsed_columns: Vec<ParsedColumn> = vec![];

                for col in columns {
                    let name = col.name.to_string();
                    /* TODO: Add datetime and timestamp here */
                    let datatype = match &col.data_type {
                        DataType::SmallInt => "int",
                        DataType::Int => "int",
                        DataType::BigInt => "int",
                        DataType::Boolean => "bool",
                        DataType::Text => "string",
                        DataType::Varchar(_bytes) => "string",
                        DataType::Float(_precision) => "float",
                        DataType::Double => "float",
                        DataType::Decimal(_precision1, _precision2) => "float",
                        DataType::Custom(ObjectName(custom_type)) => {
                            println!("custom type = {:?}", custom_type);
                            match custom_type[0].as_ref() {
                                "string" => "string",
                                "tinyint" => "int",
                                "tinyint(1)" => "int",
                                "datetime" => "string",
                                _ => "invalid",
                            }
                        }
                        _ => {
                            println!("not matched on custom type");
                            "invalid"
                        }
                    };

                    let mut is_pk: bool = false;
                    for column_option in &col.options {
                        is_pk = match column_option.option {
                            ColumnOption::Unique { is_primary } => is_primary,
                            _ => false,
                        };
                    }

                    parsed_columns.push(ParsedColumn {
                        name,
                        datatype: datatype.to_string(),
                        is_pk,
                        is_nullable: false,
                    });
                }
                //                TODO: Handle constraints,
                //    Unique, Primary Key, Nullable, Default value etc.
                for constraint in constraints {
                    println!("{:?}", constraint);
                }
                return Ok(CreateQuery {
                    table_name: table_name.to_string(),
                    columns: parsed_columns,
                });
            }

            _ => return Err("Error parsing query".to_string()),
        }
    }
}
