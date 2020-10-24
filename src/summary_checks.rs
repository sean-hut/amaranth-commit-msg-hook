use std::process::exit;

fn summary_line(content: &String) -> String {
    match content.lines().nth(0) {
        Some(summary_line) => summary_line.to_string(),
        None => {
            eprintln!("ERROR: Commit message does not have a summary line.");
            exit(2);
        }
    }
}
