use std::env::args;

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
