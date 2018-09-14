extern crate clap;
extern crate console;
extern crate rustyline;

mod meta;

use clap::{App, Arg};
use console::Style;
use meta::MetaCommandResponse;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let style_bold = Style::new().bold();
    let style_error = Style::new().red().bold();
    let error_text = style_error.apply_to("Error:");
    let matches = App::new("Simple SQLite")
        .version("0.0.1")
        .author("Ruben Paz <me@ruben.io>")
        .about("A simple sqlite implementation in Rust for fun and learning")
        .arg(
            Arg::with_name("db")
                .value_name("db_path")
                .help("The path to the database")
                .default_value("simple.db")
                .takes_value(true),
        ).get_matches();

    let db = matches.value_of("db").unwrap();

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(&format!("({})> ", style_bold.apply_to(db)));

        match readline {
            Ok(line) => {
                if line.chars().next().unwrap() == '\\' {
                    rl.add_history_entry(line.clone());
                    match meta::execute_meta_command(line) {
                        MetaCommandResponse::CommandSuccess => (),
                        MetaCommandResponse::CommandNotFound => {
                            println!("{} {}", error_text, "Command not found")
                        }
                    };
                } else {

                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                panic!(err);
            }
        }
    }
}
