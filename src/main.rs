use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

use amaranth_commit_msg_hook::check_commit_message;

fn main() {
    let arguments: Vec<String> = args().collect();

    let argument = &arguments[1];

    let content: String = match read_to_string(argument) {
        Ok(string) => string,
        Err(error) => {
            eprintln!(
                "Could not read file provided to git commit-msg hook: {}",
                error
            );
            exit(1);
        }
    };

    check_commit_message(&content);
}
