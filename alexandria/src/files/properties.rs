pub struct Properties {
    id: usize,
    st_ctime: usize,
    st_size: usize,
    st_mtime: usize,
    hash: usize,
    st_ino: usize,
    st_mode: usize,
}

impl Properties {
    pub fn create_sql() -> &'static str {
        r#"CREATE TABLE properties (
            id          INTEGER PRIMARY KEY,
            st_ctime    INTEGER NOT NULL,
            st_size     INTEGER NOT NULL,
            st_mtime    INTEGER NOT NULL,
            hash        INTEGER NOT NULL,
            st_ino      INTEGER NOT NULL,
            st_mode     INTEGER NOT NULL
        );"#
    }
}
