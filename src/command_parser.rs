use regex::Regex;
use std::collections::HashMap;

// TODO: Sanitize input string before validation
// TODO: input => lowercase => trim space from beginning and end  => replace \s* with just one \s
// TODO: => tokenize => extract table-name, columns and values => return
// TODO: => Cehck if the table exists => check if the columns exist in that table
// TODO: => Check if the values are of the correct type for given table
// TODO: => Finally Insert the data.
// TODO: => This was suppoosed to be fun.

pub fn sanitize_user_input(input: String) -> String {
    let cmd = input.to_lowercase();
    let cmd = cmd.trim();
    let cmd = Regex::new(r"\s+").unwrap().replace_all(cmd, " ");
    return cmd.to_string();
}

pub fn extract_info_from_insert_cmd(cmd: String) -> (String, Vec<String>, Vec<String>) {
    let cmd = sanitize_user_input(cmd);
    let matcher =
        Regex::new(r"[a-z]*\s*[a-z]*\s*([a-z]*)\s*\(((?:.|\n)+)\)\s*[a-z]*\s*\(((?:.|\n)+)\)")
            .unwrap();

    let captures = matcher
        .captures(&cmd)
        .expect("Error while trying to validate insert command");

    let table_name = captures.get(1).map_or("", |m| m.as_str());
    let columns = captures.get(2).map_or("", |m| m.as_str());
    let values = captures.get(3).map_or("", |m| m.as_str());

    println!(
        "table_name = {}, columns = {}, values = {}",
        table_name, columns, values
    );
    return (
        table_name.to_string(),
        columns
            .replace(" ", "")
            .split(",")
            .map(|n| n.to_string())
            .collect::<Vec<String>>(),
        values
            .replace(" ", "")
            .split(",")
            .map(|n| n.to_string())
            .collect::<Vec<String>>(),
    );
}

pub fn extract_info_from_create_table_cmd(cmd: String) -> HashMap<&'static str, String> {
    let cmd = sanitize_user_input(cmd);

    let captured_groups = Regex::new(r"create table ([a-z]*)\s+\(((?:.|\n)+)\)")
        .unwrap()
        .captures(&cmd)
        .expect("Error while trying to validate create table command");

    let table_name = captured_groups.get(1).map_or("", |m| m.as_str());
    let columns_schema = captured_groups.get(2).map_or("", |m| m.as_str());
    let mut hm: HashMap<&'static str, String> = HashMap::new();
    hm.insert("tname", String::from(table_name));
    hm.insert("columns", String::from(columns_schema));
    return hm;
}
