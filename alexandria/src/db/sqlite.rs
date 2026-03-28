use sqlite;

use crate::db::init;
use crate::logger;

pub fn init() {
    logger::debug("sqlite: starting");
    let connection = sqlite::open(":memory:").unwrap();
    for sql in init::tables() {
        logger::debug(sql);
        connection.execute(sql).unwrap();
    }
    logger::debug("sqlite: started");
}

