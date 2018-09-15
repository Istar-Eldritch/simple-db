extern crate clap;
extern crate console;
extern crate rustyline;
extern crate simpledb;

mod repl;
use clap::{App, Arg};
use simpledb::SimpleDbConfig;

fn main() {
    let matches = App::new("Simple SQLite Clone")
        .version("0.0.1")
        .author("Ruben Paz <me@ruben.io>")
        .about("A simple sqlite implementation in Rust for fun and learning")
        .arg(
            Arg::with_name("db_path")
                .value_name("db_path")
                .help("The path to the database")
                .takes_value(true),
        ).get_matches();

    let db_config = matches
        .value_of("db_path")
        .map(|db| SimpleDbConfig::new(db))
        .unwrap_or_default();

    repl::start(db_config);
}
