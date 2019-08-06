use crate::table::Table;

pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Database {
        return Database { tables: vec![] };
    }
}
