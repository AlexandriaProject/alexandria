#[macro_use]
pub mod log;
mod db;
mod platform;
mod constants;
mod files;

pub fn start() {
    db::sqlite::init();
    let path = platform::dirs::local_dir().expect("Failed");
    debug!("{:?}", path);
}

