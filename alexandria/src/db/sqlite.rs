use sqlite;

use crate::db::init;

pub fn init() {
    // FIXME: Implement proper error handling here, unwrap is not safe.
    debug!("sqlite: starting");
    let connection = sqlite::open(":memory:").unwrap();
    connection.execute("PRAGMA foreign_keys = ON;").unwrap();
    for sql in init::tables() {
        debug!("{sql}");
        connection.execute(sql).unwrap();
    }
    debug!("sqlite: started");
}

