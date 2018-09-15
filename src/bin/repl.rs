use console::Style;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use simpledb::SimpleDbConfig;
use std::fmt;
use std::process;

struct REPLCommand<'a> {
    pub cmd: &'a str,
    pub help: &'a str,
    pub is_match: &'a Fn(&str) -> bool,
    pub execute: &'a Fn((&SimpleDbConfig, &str)) -> (),
}

impl<'a> fmt::Display for REPLCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{cmd:<width$}{help}",
            cmd = self.cmd,
            width = 10,
            help = self.help
        )
    }
}

fn print_help() -> () {
    let result: String = COMMANDS
        .iter()
        .map(|command| format!("\t{}", command))
        .fold(String::new(), |acc, next| format!("{}\n{}", acc, next));
    println!("\nAvailable commands:  {}\n", result)
}

const COMMANDS: [REPLCommand; 3] = [
    REPLCommand {
        cmd: "[SQL]",
        help: "Executes a query agains the db",
        is_match: &|_| false,
        execute: &|_| panic!("Not yet implemented"),
    },
    REPLCommand {
        cmd: "\\help",
        help: "Prints this help page",
        is_match: &|candidate| candidate == "\\help",
        execute: &|_| {
            print_help();
        },
    },
    REPLCommand {
        cmd: "\\q",
        help: "Exits the REPL",
        is_match: &|candidate| candidate == "\\q",
        execute: &|_| {
            process::exit(0);
        },
    },
];

fn handle_meta_commands(db_config: &SimpleDbConfig, candidate: &str) -> () {
    for command in COMMANDS.iter() {
        let is_match = command.is_match;
        if is_match(candidate) {
            let execute = command.execute;
            return execute((db_config, candidate));
        }
    }

    let error_text = Style::new().red().bold().apply_to("Error:");
    println!("{} Command \"{}\" not recognized", error_text, candidate);
    print_help();
}

pub fn start(db_config: SimpleDbConfig) {
    let mut rl = Editor::<()>::new();
    let style_bold = Style::new().bold();
    loop {
        let readline = rl.readline(&format!("({})> ", style_bold.apply_to(&db_config.db_name)));

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.clone());
                handle_meta_commands(&db_config, &line);
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                panic!(err);
            }
        }
    }
}
