pub struct Summary<'a> {
    summary_line: Result<&'a str, &'a str>,
    category_abbreviation: Result<&'a str, &'a str>,
    first_word: Result<&'a str, &'a str>,
}

pub fn summary(content: &str) -> Summary {
    Summary {
        summary_line: summary_line(&content),
        category_abbreviation: category_abbreviation(&content),
        first_word: first_word(&content),
    }
}

        }
    }
}

pub fn first_summary_word_not_lowercase(content: &str) -> Result<String, String> {
    match first_summary_word(content).to_ascii_lowercase() == first_summary_word(content) {
        true => Ok("First word in summary line is lowercase.".to_string()),
        false => Err("First word in summary line is not lowercase.".to_string()),
    }
}

pub fn first_summary_word_not_imperative_mood(content: &str) -> Result<String, String> {
    match first_summary_word(content).ends_with("ed") || first_summary_word(content).ends_with("es")
    {
        true => Err("Summary does not use imperative mood.".to_string()),
        false => Ok("Summary uses imperative mood.".to_string()),
    }
}

pub fn summary_over_50_characters(content: &str) -> Result<String, String> {
    if summary_line(content).len() < 50 {
        Ok("Summary line is less than 50 characters.".to_string())
    } else {
        Err("Summary line is more than 50 characters.".to_string())
    }
}

pub fn summary_ends_with_period(content: &str) -> Result<String, String> {
    let ends_with_period: bool = summary_line(&content).ends_with('.');

    match ends_with_period {
        false => Ok("Summary line does not end in a period.".to_string()),
        true => Err("Summary line end in a period.".to_string()),
fn summary_line(content: &str) -> Result<&str, &str> {
    match content.lines().next() {
        Some(s) => Ok(s),
        None => Err("Commit message does not have a summary line."),
    }
}

pub fn invalid_category_abbreviation(content: &str) -> Result<String, String> {
    let category_abbreviations: Vec<String> = vec![
        "AA".to_string(),
        "AR".to_string(),
        "B".to_string(),
        "BF".to_string(),
        "C".to_string(),
        "CT".to_string(),
        "D".to_string(),
        "DD".to_string(),
        "IV".to_string(),
        "M".to_string(),
        "R".to_string(),
        "RD".to_string(),
        "RV".to_string(),
        "T".to_string(),
    ];

    let category_abbreviation: String = match summary_line(&content).split_ascii_whitespace().next()
    {
        Some(s) => s.to_string(),
        None => {
            eprintln!(
                "ERROR: Commit message summary line needs to start with a category abbreviation."
            );
            exit(2);
        }
    };

    let valid_category_abbreviation: bool = category_abbreviations
        .iter()
        .any(|x| x == &category_abbreviation);

    match valid_category_abbreviation {
        true => Ok("Valid category abbreviation.".to_string()),
        false => Err("Invalid category abbreviation.".to_string()),
fn first_word(content: &str) -> Result<&str, &str> {
    match summary_line(&content) {
        Ok(s) => match s.split_ascii_whitespace().nth(1) {
            Some(s) => Ok(s),
            None => Err(
                "Commit message summary line must to be more than just a category abbreviation.",
            ),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod summary {

    use super::{
        first_summary_word_not_imperative_mood, first_summary_word_not_lowercase,
        invalid_category_abbreviation, summary_ends_with_period, summary_over_50_characters,
    };

    #[test]
    fn first_summary_word_not_lowercase_test() -> Result<(), &'static str> {
        let not_lowercase_first_word: String = "D Add readme".to_string();

        match first_summary_word_not_lowercase(&not_lowercase_first_word) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }

    #[test]
    fn first_summary_word_not_imperative_mood_added_test() -> Result<(), &'static str> {
        let not_imperative_mood_added: String = "D added readme".to_string();

        match first_summary_word_not_imperative_mood(&not_imperative_mood_added) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }

    #[test]
    fn first_summary_word_not_imperative_mood_addes_test() -> Result<(), &'static str> {
        let not_imperative_mood_addes: String = "D addes readme".to_string();

        match first_summary_word_not_imperative_mood(&not_imperative_mood_addes) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }

    #[test]
    fn summary_over_50_characters_test() -> Result<(), &'static str> {
        let summary_line_to_long: String =
            "D add overview, dependencies, changelog, license to readme".to_string();

        match summary_over_50_characters(&summary_line_to_long) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }

    #[test]
    fn summary_ends_with_period_test() -> Result<(), &'static str> {
        let summary_ends_in_period: String = "D add readme.".to_string();

        match summary_ends_with_period(&summary_ends_in_period) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }

    #[test]
    fn invalid_category_abbreviation_test() -> Result<(), &'static str> {
        let category_abbreviation_invalid: String = "Z add readme".to_string();

        match invalid_category_abbreviation(&category_abbreviation_invalid) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected."),
        }
    }
}
