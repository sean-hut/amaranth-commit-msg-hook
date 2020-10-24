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

pub fn first_summary_word_not_lowercase(content: &String) -> Result<String, String> {
    match first_summary_word(content).to_ascii_lowercase() == first_summary_word(content) {
        true => Ok("First word in summary line is lowercase.".to_string()),
        false => Err("First word in summary line is not lowercase.".to_string()),
    }
}

#[cfg(test)]
mod summary {

    use super::{
        first_summary_word_not_imperative_mood, first_summary_word_not_lowercase,
        invalid_category_abbreviation, summary_ends_with_period, summary_over_50_characters,
    };

    use super::first_summary_word_not_lowercase;

    #[test]
    fn first_summary_word_not_lowercase_test() -> Result<(), &'static str> {
        let not_lowercase_first_word: String = "D Add readme".to_string();

        match first_summary_word_not_lowercase(&not_lowercase_first_word) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }
}
