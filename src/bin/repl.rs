use console::Style;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use simpledb::SimpleDbConfig;

pub fn start(db_config: SimpleDbConfig) {
    let mut rl = Editor::<()>::new();
    let style_bold = Style::new().bold();
    let style_error = Style::new().red().bold();
    let error_text = style_error.apply_to("Error:");
    loop {
        let readline = rl.readline(&format!("({})> ", style_bold.apply_to(db_config.db_name)));

        match readline {
            Ok(line) => {
                if line.chars().next().unwrap() == '\\' {
                    rl.add_history_entry(line.clone());
                    println!("{} {}", error_text, "Command not found")
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
