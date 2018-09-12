extern crate clap;
extern crate console;

use clap::{App, Arg};
use console::Style;
use console::Term;
use std::env;

fn main() {
    let term = Term::stdout();
    let pwd: String = env::current_dir().unwrap().to_str().unwrap().into();
    let default_db_path = format!("{}/simple.db", &pwd);

    let style_bold = Style::new().bold();
    let matches = App::new("Simple SQLite")
        .version("0.0.1")
        .author("Ruben Paz <me@ruben.io>")
        .about("A simple sqlite implementation in Rust for fun and learning")
        .arg(
            Arg::with_name("db")
                .value_name("db_path")
                .help("The path to the database")
                .default_value(&default_db_path)
                .takes_value(true),
        ).get_matches();

    let db = matches.value_of("db").unwrap();

    term.write_line(&format!("{} {}", style_bold.apply_to("Selected db:"), db))
        .unwrap();
}
