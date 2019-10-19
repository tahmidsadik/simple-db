use sqlparser::ast::{
    ColumnDef, DataType::Custom, Expr, FileFormat, ObjectName, SetExpr, SqlOption, Statement,
    Statement::CreateTable, TableConstraint,
};
use sqlparser::dialect::GenericDialect;
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
    pub fn new(query: &str) -> Result<CreateQuery, String> {
        let dialect = GenericDialect {};
        let statement = &Parser::parse_sql(&dialect, query.to_string()).unwrap()[0];

        match statement {
            CreateTable {
                name,
                columns,
                constraints,
                with_options,
                external,
                file_format,
                location,
            } => {
                println!("table name = {}, ", name);
                let table_name = name;
                let mut parsed_columns: Vec<ParsedColumn> = vec![];
                for col in columns {
                    let name = col.name.to_string();
                    let datatype = match &col.data_type {
                        Int => "int",
                        Boolean => "bool",
                        Text => "text",
                        Float => "float",
                        Custom(ObjectName(custom_type)) => {
                            if custom_type[0] == "string" {
                                "string"
                            } else {
                                "invalid"
                            }
                        }
                    };
                    parsed_columns.push(ParsedColumn {
                        name,
                        datatype: datatype.to_string(),
                        is_pk: false,
                        is_nullable: false,
                    });
                }

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

        // match table_name {
        //     Some(name) => Ok(SelectQuery {
        //         from: name,
        //         projection,
        //         where_expressions: vec![],
        //     }),
        //     None => Err(
        //         "Error while trying to parser select statement. Cannot extract table name"
        //             .to_string(),
        //     ),
        // }
    }
}
