pub struct Tag {
    id: usize,
    namespace: String,
    value: String,
}

impl Tag {
    pub fn create_sql() -> &'static str {
        r#"CREATE TABLE tag (
            id          INTEGER PRIMARY KEY,
            namespace   TEXT NOT NULL,
            name        TEXT NOT NULL,
            UNIQUE (namespace, name)
        );"#
    }
}
