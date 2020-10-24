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

fn first_summary_word(content: &String) -> String {
    match summary_line(&content).split_ascii_whitespace().nth(1) {
        Some(s) => s.to_string(),
        None => {
            eprintln!("ERROR: Commit message summary line needs to be more than just a category abbreviation.");
            exit(2);
        }
    }
}
