mod log;
mod db;
mod platform;
mod constants;

use log::logger;
pub fn start() {
    logger::info("Starting");
    db::sqlite::init();
    let path = platform::dirs::local_dir().expect("Failed");
    logger::debug(path.to_string_lossy().as_ref());
}

