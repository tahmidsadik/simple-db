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
        let parsed_cmd_hm = extract_info_from_create_table_cmd(input);
        let table_name = String::from(parsed_cmd_hm.get("tname").unwrap());
        let columns_schema = String::from(parsed_cmd_hm.get("columns").unwrap());

        let expected_table_name = String::from("users");
        let expected_columns_schema = String::from("id int, name string");
        assert_eq!(table_name, expected_table_name);
        assert_eq!(columns_schema, expected_columns_schema);
    }
}
