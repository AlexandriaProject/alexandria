use sqlite;

use crate::db::init;
use crate::logger;

pub fn init() {
    // FIXME: Implement proper error handling here, unwrap is not safe.
    logger::debug("sqlite: starting");
    let connection = sqlite::open(":memory:").unwrap();
    connection.execute("PRAGMA foreign_keys = ON;").unwrap();
    for sql in init::tables() {
        logger::debug(sql);
        connection.execute(sql).unwrap();
    }
    logger::debug("sqlite: started");
}

