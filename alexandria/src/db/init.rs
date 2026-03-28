use crate::files::*;

pub fn tables() -> Vec<&'static str> {
    vec![
        Properties::create_sql(),
        File::create_sql(),
        // Tag::create_sql(),
    ]
}
