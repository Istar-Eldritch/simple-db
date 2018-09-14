pub enum MetaCommandResponse {
    CommandSuccess,
    CommandNotFound,
}

pub fn execute_meta_command(cmd: String) -> MetaCommandResponse {
    match cmd {
        _ => MetaCommandResponse::CommandNotFound,
    }
}
