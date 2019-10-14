use regex::Regex;
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement, TableFactor,
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

pub fn sanitize_user_input(input: String) -> String {
    let cmd = input.to_lowercase();
    let cmd = cmd.trim();
    let cmd = Regex::new(r"\s+").unwrap().replace_all(cmd, " ");
    return cmd.to_string();
}

pub fn extract_info_from_insert_cmd(cmd: String) -> (String, Vec<String>, Vec<Vec<String>>) {
    let cmd = sanitize_user_input(cmd);
    let matcher =
        Regex::new(r"[a-z]*\s*[a-z]*\s*([a-z]*)\s*\(((?:.|\n)+)\)\s*[a-z]*\s*(\((?:.|\n)+\))")
            .unwrap();
    let values_matcher = Regex::new(r"\(((\d|[a-z]|[A-Z]|,|\s|')*)\)").unwrap();

    let captures = matcher
        .captures(&cmd)
        .expect("Error while trying to validate insert command");

    let table_name = captures.get(1).map_or("", |m| m.as_str());
    let columns = captures.get(2).map_or("", |m| m.as_str());
    let values_list = captures.get(3).map_or("", |m| m.as_str());

    let captures = values_matcher.captures_iter(&values_list);
    let mut values: Vec<String> = vec![];
    for c in captures {
        values.push(c[1].to_string());
    }

    println!(
        "table_name = {}, columns = {}, values = {}",
        table_name, columns, values_list
    );
    return (
        table_name.to_string(),
        columns
            .replace(" ", "")
            .split(",")
            .map(|n| n.to_string())
            .collect::<Vec<String>>(),
        values
            .iter()
            .map(|v| {
                return v
                    .replace(" ", "")
                    .replace("\"", "")
                    .replace("'", "")
                    .split(",")
                    .map(|sv| sv.to_string())
                    .collect::<Vec<String>>();
            })
            .collect::<Vec<Vec<String>>>(),
    );
}

pub fn extract_info_from_create_table_cmd(cmd: String) -> (String, Vec<ParsedColumn>) {
    let cmd = sanitize_user_input(cmd);

    let captured_groups = Regex::new(r"create table ([a-z]*)\s*\(((?:.|\n)+)\)")
        .unwrap()
        .captures(&cmd)
        .expect("Error while trying to validate create table command");

    let table_name = captured_groups.get(1).map_or("", |m| m.as_str());
    let parsed_columns = captured_groups.get(2).map_or("", |m| m.as_str());

    let splitted_columns: Vec<String> = parsed_columns
        .split(",")
        .map(|c| c.to_lowercase())
        .collect();

    let mut parsed_columns: Vec<ParsedColumn> = vec![];
    for c in splitted_columns {
        let is_pk = c.contains("primary key");
        let is_nullable = c.contains("not null");

        let c = c.replace("primary key", "");
        let c = c.replace("not null", "");

        if let [name, datatype] = c.trim().split(" ").collect::<Vec<&str>>()[..] {
            parsed_columns.push(ParsedColumn {
                name: name.to_string(),
                datatype: datatype.to_string(),
                is_pk,
                is_nullable,
            });
        };
    }

    return (String::from(table_name), parsed_columns);
}

pub fn extract_info_from_select_statements(sql: &str) {
    let dialect = GenericDialect {};
    let statement = &Parser::parse_sql(&dialect, sql.to_string()).unwrap()[0];
    match statement {
        Statement::Query(bq) => match &(*bq).body {
            SetExpr::Select(select) => {
                for p in &(*select).projection {
                    match p {
                        UnnamedExpr(exp) => {
                            println!("unnamed expr {}", exp);
                        }
                        QualifiedWildcard(obj_name) => {
                            println!("objname = {}", obj_name);
                        }
                        Wildcard => {
                            println!("wildcard");
                        }
                        ExprWithAlias { expr, alias } => {
                            println!("expression = {} alias = {}", expr, alias);
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
                            println!("Table name = {}", name);
                            match alias {
                                Some(alias) => println!("alias = {}", alias),
                                None => println!("No alias"),
                            }
                        }
                        _ => println!("Nested join or derived tables"),
                    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_input_trims_single_whitespaces_correctly_from_start_and_end() {
        let input = String::from(" hello ");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_trims_multiple_whitespaces_correctly_from_start_and_end() {
        let input = String::from("         hello         ");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_lowercases_the_input() {
        let input = String::from("HELLO WORLD GoodBye World");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello world goodbye world");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn sanitize_input_replaces_multiple_whitespaces_into_single_one_inside_string() {
        // should turn "hello     world       end" => "hello world end"
        let input = String::from("hello       world        end");
        let sanitized_input = sanitize_user_input(input);
        let expected_output = String::from("hello world end");
        assert_eq!(sanitized_input, expected_output);
    }

    #[test]
    fn parses_correctly_from_create_table_cmd() {
        let input = String::from("CREATE TABLE users (id int, name string)");
        let (table_name, columns) = extract_info_from_create_table_cmd(input);

        let expected_table_name = String::from("users");
        let expected_columns = vec![
            ParsedColumn {
                name: "id".to_string(),
                datatype: "int".to_string(),
                is_pk: false,
                is_nullable: false,
            },
            ParsedColumn {
                name: "name".to_string(),
                datatype: "string".to_string(),
                is_pk: false,
                is_nullable: false,
            },
        ];
        assert_eq!(table_name, expected_table_name);
        assert_eq!(columns, expected_columns);
    }
    #[test]
    fn parses_correctly_from_create_table_cmd_no_whitespace() {
        let input = String::from("CREATE TABLE users(id int,name string)");
        let (table_name, columns) = extract_info_from_create_table_cmd(input);

        let expected_table_name = String::from("users");
        let expected_columns = vec![
            ParsedColumn {
                name: "id".to_string(),
                datatype: "int".to_string(),
                is_pk: false,
                is_nullable: false,
            },
            ParsedColumn {
                name: "name".to_string(),
                datatype: "string".to_string(),
                is_pk: false,
                is_nullable: false,
            },
        ];
        assert_eq!(table_name, expected_table_name);
        assert_eq!(columns, expected_columns);
    }

    #[test]
    fn tests_create_table_command_with_primary_key() {
        let input = String::from("CREATE TABLE users(id int PRIMARY KEY,name string)");
        let (table_name, columns) = extract_info_from_create_table_cmd(input);

        let expected_table_name = String::from("users");
        let expected_columns = vec![
            ParsedColumn {
                name: "id".to_string(),
                datatype: "int".to_string(),
                is_pk: true,
                is_nullable: false,
            },
            ParsedColumn {
                name: "name".to_string(),
                datatype: "string".to_string(),
                is_pk: false,
                is_nullable: false,
            },
        ];

        assert_eq!(table_name, expected_table_name);
        assert_eq!(columns, expected_columns);
    }

    #[test]
    fn tests_extract_info_from_insert_cmd() {
        let input = String::from("insert into users (id, name, age) values(1, 'hello', 27);");
        let (table, columns, values) = extract_info_from_insert_cmd(input);
        assert_eq!(table, "users");
        assert_eq!(
            columns,
            vec!["id".to_string(), "name".to_string(), "age".to_string()]
        );
        assert_eq!(
            values,
            vec![vec!["1".to_string(), "hello".to_string(), "27".to_string()]]
        );
    }

    #[test]
    fn tests_extract_info_from_insert_cmd_with_multiple_values() {
        let input = String::from(
            "insert into users (id, name, age) values(1, 'hello', 27),(2, 'world', 13);",
        );
        let (table, columns, values) = extract_info_from_insert_cmd(input);
        assert_eq!(table, "users");
        assert_eq!(
            columns,
            vec!["id".to_string(), "name".to_string(), "age".to_string()]
        );
        assert_eq!(
            values,
            vec![
                vec!["1".to_string(), "hello".to_string(), "27".to_string()],
                vec!["2".to_string(), "world".to_string(), "13".to_string()]
            ]
        );
    }

    #[test]
    fn tests_parsing_select_statement() {
        let sql = "select * from users";
        let y = extract_info_from_select_statements(&sql);
        assert_eq!(1, 1);
    }
}
