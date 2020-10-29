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

impl<'a> Summary<'a> {
    pub fn category_abbreviation(&self) -> Result<&'a str, &'a str> {
        let category_abbreviations: Vec<&str> = vec![
            "AA", "AR", "B", "BF", "C", "CT", "D", "DD", "IV", "M", "R", "RD", "RV", "T",
        ];

        match self.category_abbreviation {
            Ok(s) => match category_abbreviations.iter().any(|x| x == &s) {
                true => Ok("Valid category abbreviation."),
                false => Err("Invalid category abbreviation."),
            },
            Err(e) => Err(e),
        }
    }

    pub fn first_word_lowercase(&self) -> Result<&'a str, &'a str> {
        match self.first_word {
            Ok(s) => match &s.to_ascii_lowercase()[..] == s {
                true => Ok("First word in summary line is lowercase."),
                false => Err("First word in summary line is not lowercase."),
            },
            Err(e) => Err(e),
        }
    }

    pub fn first_word_imperative_mood(&self) -> Result<&'a str, &'a str> {
        match self.first_word {
            Ok(s) => match s.ends_with("ed") || s.ends_with("es") {
                true => Err("Summary does not use imperative mood."),
                false => Ok("Summary uses imperative mood."),
            },
            Err(e) => Err(e),
        }
    }

    pub fn period(&self) -> Result<&'a str, &'a str> {
        match self.summary_line {
            Ok(s) => match s.ends_with('.') {
                false => Ok("Summary line does not end in a period."),
                true => Err("Summary line end in a period."),
            },
            Err(e) => Err(e),
        }
    }

    pub fn over_max_length(&self) -> Result<&'a str, &'a str> {
        match self.summary_line {
            Ok(s) => match s.len() < 50 {
                true => Ok("Summary line is less than 50 characters."),
                false => Err("Summary line is more than 50 characters."),
            },
            Err(e) => Err(e),
        }
    }
}

fn summary_line(content: &str) -> Result<&str, &str> {
    match content.lines().next() {
        Some(s) => Ok(s),
        None => Err("Commit message does not have a summary line."),
    }
}

fn category_abbreviation(content: &str) -> Result<&str, &str> {
    match summary_line(&content) {
        Ok(s) => match s.split_ascii_whitespace().next() {
            Some(s) => Ok(s),
            None => Err("Commit message summary line needs to start with a category abbreviation."),
        },

        Err(e) => Err(e),
    }
}

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
