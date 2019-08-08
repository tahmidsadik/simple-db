use regex::Regex;

// TODO: Sanitize input string before validation
// TODO: input => lowercase => trim space from beginning and end  => replace \s* with just one \s
// TODO: => tokenize => extract table-name, columns and values => return
// TODO: => Cehck if the table exists => check if the columns exist in that table
// TODO: => Check if the values are of the correct type for given table
// TODO: => Finally Insert the data.
// TODO: => This was suppoosed to be fun.
pub fn extract_info_from_insert_cmd(cmd: String) {
    let matcher =
        Regex::new(r"[a-z]*\s*[a-z]*\s*([a-z]*)\s*\(((?:.|\n)+)\)\s*[a-z]*\s*\(((?:.|\n)+)\)")
            .unwrap();

    println!("regex created");

    let captures = matcher
        .captures(&cmd)
        .expect("Error while trying to validate insert command");

    println!("Capture complete");

    let table_name = captures.get(1).map_or("", |m| m.as_str());
    let columns = captures.get(2).map_or("", |m| m.as_str());
    let values = captures.get(3).map_or("", |m| m.as_str());

    println!(
        "table_name = {}, columns = {}, values = {}",
        table_name, columns, values
    );
}
