use crate::files::*;

pub fn tables() -> Vec<&'static str> {
    // FIXME: It might be nice to have some reference to which table was 
    // created with what sql query, might use a struct for this later.
    vec![
        Properties::create_sql(),
        Tag::create_sql(),
        File::create_sql(),
    ]
}
