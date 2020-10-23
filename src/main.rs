use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

use amaranth_commit_msg_hook::check_commit_message;

fn main() {
    let argument: Option<String> = args().nth(1);

    check_argument(argument);
}

fn check_argument(argument: Option<String>) {
    match argument {
        Some(path_string) => read_commit_message(path_string),

        None => {
            eprintln!("Did not recieve an argument from git.");
            exit(1);
        }
    }
}

fn read_commit_message(path_string: String) {
    let path: &Path = Path::new(&path_string);
    let commit_message_contents = read_to_string(path);

    match commit_message_contents {
        Ok(contents) => check_commit_message(contents),

        Err(error) => {
            eprintln!(
                "Could not read file provided to git commit-msg hook: {}",
                error
            );
            exit(1);
        }
    }
}
