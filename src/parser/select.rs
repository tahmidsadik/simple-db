use sqlparser::ast::{
    Expr,
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement, TableFactor, Value,
};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum Binary {
    Eq,
    Lt,
    Gt,
}

#[derive(Debug)]
pub enum Operator {
    Unary,
    Binary(Binary),
    Ternary,
    None,
}

// Only binary operators for now
#[derive(Debug)]
pub struct Expression {
    pub left: String,
    pub right: String,
    pub op: Operator,
}

#[derive(Debug)]
pub struct SelectQuery {
    pub from: String,            // table name
    pub projection: Vec<String>, // columns that will be fetched
    pub where_expressions: Vec<Expression>,
}

impl SelectQuery {
    pub fn new(query: &str) -> Result<SelectQuery, String> {
        let dialect = MySqlDialect {};
        let statement = &Parser::parse_sql(&dialect, query.to_string()).unwrap()[0];

        let mut table_name: Option<String> = None;
        let mut projection: Vec<String> = vec![];
        let mut where_expressions: Vec<Expression> = vec![];

        match statement {
            Statement::Query(bq) => match &(*bq).body {
                SetExpr::Select(select) => {
                    for p in &(*select).projection {
                        match p {
                            UnnamedExpr(exp) => match exp {
                                Expr::Identifier(i) => {
                                    projection.push(i.to_string());
                                }
                                _ => {
                                    println!(
                                        "Failing to parse expression in the where clause.\
                                         It's probably not an identifier or a value.\
                                         Cannot parse nested expressions :( ."
                                    );
                                }
                            },
                            QualifiedWildcard(obj_name) => {
                                println!("Found qualified wildcard in the expression. Wildcard name is  {}", obj_name);
                            }
                            Wildcard => {
                                projection.push("*".to_string());
                            }
                            ExprWithAlias { expr, alias } => {
                                println!("expression = {} alias = {}", expr, alias);
                                match expr {
                                    Expr::Identifier(i) => {
                                        projection.push(i.to_string());
                                    }
                                    _ => {
                                        println!("Detected expression with alias. Cannot parse expression with alias.");
                                    }
                                }
                            }
                        }
                    }

                    for f in &(*select).from {
                        match &f.relation {
                            TableFactor::Table {
                                name,
                                alias,
                                args: _args,
                                with_hints: _with_hints,
                            } => {
                                table_name = Some(name.to_string());
                                match alias {
                                    Some(alias) => println!("alias = {}", alias),
                                    None => println!("No alias"),
                                }
                            }
                            _ => println!("Nested join or derived tables"),
                        }
                    }

                    match &(*select).selection {
                        Some(where_expression) => {
                            println!("{:?}", where_expression);
                            match where_expression {
                                Expr::BinaryOp { left, op, right } => {
                                    if let Expr::Identifier(col_name) = &(**left) {
                                        if let Expr::Value(v) = &(**right) {
                                            if let Value::Number(n) = v {
                                                let bo = match op {
                                                    sqlparser::ast::BinaryOperator::Eq => {
                                                        where_expressions.push(Expression {
                                                            left: col_name.to_string(),
                                                            right: n.to_string(),
                                                            op: Operator::Binary(Binary::Eq),
                                                        });
                                                    }
                                                    sqlparser::ast::BinaryOperator::Gt => {
                                                        where_expressions.push(Expression {
                                                            left: col_name.to_string(),
                                                            right: n.to_string(),
                                                            op: Operator::Binary(Binary::Gt),
                                                        });
                                                    }
                                                    sqlparser::ast::BinaryOperator::Lt => {
                                                        where_expressions.push(Expression {
                                                            left: col_name.to_string(),
                                                            right: n.to_string(),
                                                            op: Operator::Binary(Binary::Lt),
                                                        });
                                                    }
                                                    _ => {
                                                        panic!("cannot parse select query");
                                                    }
                                                };
                                            }
                                        }
                                    };
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
                _ => {
                    println!("don't care");
                }
            },
            _ => {
                println!("don't care");
            }
        }

        match table_name {
            Some(name) => Ok(SelectQuery {
                from: name,
                projection,
                where_expressions,
            }),
            None => Err(
                "Error while trying to parse select statement. Cannot extract table name"
                    .to_string(),
            ),
        }
    }
}
