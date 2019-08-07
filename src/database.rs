use crate::table::Table;

pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Database {
        return Database { tables: vec![] };
    }

    pub fn table_exists(&self, tname: String) -> bool {
        self.tables.iter().any(|t| t.name == tname)
    }
}
