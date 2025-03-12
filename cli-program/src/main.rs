use cli_program::arguments::{self, Command};

fn main() {
    let cmd = Command::parse_arguments();
    match cmd.cmd {
        arguments::CMD::Read => {
            cmd.read_file();
        },
        arguments::CMD::Append => {
            cmd.append_to_file();
        }
        _ => {}
    }
}