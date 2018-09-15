#![crate_name = "simpledb"]

use std::path::Path;

pub struct SimpleDbConfig<'a> {
    pub db_name: &'a str,
    pub db_path: &'a Path,
}

impl<'a> SimpleDbConfig<'a> {
    pub fn new(db_path: &'a str) -> SimpleDbConfig<'a> {
        let db_path = Path::new(db_path);
        let db_name = db_path.file_name().unwrap().to_str().unwrap();
        SimpleDbConfig { db_name, db_path }
    }
}

impl<'a> Default for SimpleDbConfig<'a> {
    fn default() -> SimpleDbConfig<'a> {
        SimpleDbConfig {
            db_name: "simple.db",
            db_path: Path::new("./simple.db"),
        }
    }
}
