use std::time;
use crate::files::Properties;

pub struct File {
    id: usize,
    path: String,
    created_at: time::Instant,
    updated_at: time::Instant,
    props: Properties
}

impl File {
    pub fn create_sql() -> &'static str {
        r#"CREATE TABLE file (
            id          INTEGER PRIMARY KEY,
            path        TEXT NOT NULL UNIQUE,
            created_at  INTEGER NOT NULL,
            updated_at  INTEGER NOT NULL,
            props_id    INTEGER NOT NULL,
            FOREIGN KEY (props_id) REFERENCES properties(id)
        );"#
    }
}

