use sqlparser::ast::{Expr, Query, SelectItem, SetExpr, Statement, Value, Values};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::str::FromStr;

pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl InsertQuery {
    fn new(table_name: String, columns: Vec<String>, values: Vec<Vec<String>>) -> InsertQuery {
        InsertQuery {
            table_name,
            columns,
            values,
        }
    }
}

impl FromStr for InsertQuery {
    type Err = String;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        let dialect = MySqlDialect {};
        let statement = &Parser::parse_sql(&dialect, query.to_string()).unwrap()[0];

        let mut tname: Option<String> = None;
        let mut columns: Vec<String> = vec![];
        let mut all_vals: Vec<Vec<String>> = vec![];

        if let Statement::Insert {
            table_name,
            columns: cols,
            source,
        } = statement
        {
            tname = Some(table_name.to_string());
            for c in cols {
                columns.push(c.to_string());
            }
            match &**source {
                Query {
                    ctes: _ctes,
                    body,
                    order_by: _order_by,
                    limit: _limit,
                    offset: _offset,
                    fetch: _fetch,
                } => {
                    if let SetExpr::Values(values) = body {
                        if let Values(expressions) = values {
                            for i in expressions {
                                let mut value_set: Vec<String> = vec![];
                                for e in i {
                                    match e {
                                        Expr::Value(v) => match v {
                                            Value::Number(n) => {
                                                value_set.push(n.to_string());
                                            }
                                            Value::Boolean(b) => match *b {
                                                true => value_set.push("true".to_string()),
                                                false => value_set.push("false".to_string()),
                                            },
                                            _ => {}
                                        },
                                        Expr::Identifier(i) => {
                                            value_set.push(i.to_string());
                                        }
                                        _ => {}
                                    }
                                }
                                all_vals.push(value_set);
                            }
                        }
                    }
                }
            }
        }

        match tname {
            Some(t) => Ok(InsertQuery {
                table_name: t,
                columns,
                values: all_vals,
            }),
            None => Err(String::from("Cannot parse insert query")),
        }
    }
}
