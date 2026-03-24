use sqlite;

use crate::logger;

pub fn init() {
    logger::debug("sqlite: starting");
    let connection = sqlite::open(":memory:").unwrap();
    let query = "CREATE TABLE Files(name TEXT);";
    connection.execute(query).unwrap();
    logger::debug("sqlite: started");
}

