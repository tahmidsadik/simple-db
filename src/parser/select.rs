use sqlparser::ast::{
    Expr,
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement, TableFactor,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
pub enum BinaryOperator {
    Eq,
    Lt,
    Gt,
}

#[derive(Debug)]
pub enum Operator {
    UnaryOperator,
    BinaryOperator,
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
    pub where_expressions: Vec<Operator>,
}

impl SelectQuery {
    pub fn new(query: &str) -> Result<SelectQuery, String> {
        let dialect = GenericDialect {};
        let statement = &Parser::parse_sql(&dialect, query.to_string()).unwrap()[0];

        let mut table_name: Option<String> = None;
        let mut projection: Vec<String> = vec![];

        match statement {
            Statement::Query(bq) => match &(*bq).body {
                SetExpr::Select(select) => {
                    for p in &(*select).projection {
                        match p {
                            UnnamedExpr(exp) => {
                                println!("unnamed expr {}", exp);
                                match exp {
                                    Expr::Identifier(i) => {
                                        projection.push(i.to_string());
                                    }
                                    _ => {
                                        println!("not ident");
                                    }
                                }
                            }
                            QualifiedWildcard(obj_name) => {
                                println!("objname = {}", obj_name);
                            }
                            Wildcard => {
                                println!("wildcard");
                                projection.push("*".to_string());
                            }
                            ExprWithAlias { expr, alias } => {
                                println!("expression = {} alias = {}", expr, alias);
                                match expr {
                                    Expr::Identifier(i) => {
                                        projection.push(i.to_string());
                                    }
                                    _ => {
                                        println!("not ident");
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
                                args,
                                with_hints,
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
                where_expressions: vec![],
            }),
            None => Err(
                "Error while trying to parser select statement. Cannot extract table name"
                    .to_string(),
            ),
        }
    }
}
