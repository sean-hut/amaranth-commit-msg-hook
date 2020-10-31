pub struct Summary<'a> {
    summary_line: Result<&'a str, &'a str>,
    category_abbreviation: Result<&'a str, &'a str>,
    first_word: Result<&'a str, &'a str>,
}

impl<'a> Summary<'a> {
    pub fn summary(content: &str) -> Summary {
        Summary {
            summary_line: Summary::summary_line(&content),
            category_abbreviation: Summary::category_abbreviation(&content),
            first_word: Summary::first_word(&content),
        }
    }

    fn summary_line(content: &str) -> Result<&str, &str> {
        match content.lines().next() {
            Some(s) => Ok(s),
            None => Err("Commit message does not have a summary line."),
        }
    }

    fn category_abbreviation(content: &str) -> Result<&str, &str> {
        match Summary::summary_line(&content) {
            Ok(s) => match s.split_ascii_whitespace().next() {
                Some(s) => Ok(s),
                None => {
                    Err("Commit message summary line needs to start with a category abbreviation.")
                }
            },

            Err(e) => Err(e),
        }
    }

    fn first_word(content: &str) -> Result<&str, &str> {
        match Summary::summary_line(&content) {
            Ok(s) => match s.split_ascii_whitespace().nth(1) {
                Some(s) => Ok(s),
                None => Err(
                    "Commit message summary line must to be more than just a category abbreviation.",
                ),
            },
            Err(e) => Err(e),
        }
    }

    pub fn valid_category_abbreviation(&self) -> Result<&'a str, &'a str> {
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

#[cfg(test)]
mod summary {

    use super::Summary;

    #[test]
    fn first_word_lowercase_test() -> Result<(), ()> {
        match Summary::summary("D Add readme").first_word_lowercase() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn first_word_imperative_mood_added_test() -> Result<(), ()> {
        match Summary::summary("D added readme").first_word_imperative_mood() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn first_word_imperative_mood_fixes_test() -> Result<(), ()> {
        match Summary::summary("D fixes readme").first_word_imperative_mood() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn over_max_length_test() -> Result<(), ()> {
        match Summary::summary("D add overview, dependencies, changelog, license to readme")
            .over_max_length()
        {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn period_test() -> Result<(), ()> {
        match Summary::summary("D add readme.").period() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn valid_category_abbreviation_test() -> Result<(), ()> {
        match Summary::summary("Z add readme").valid_category_abbreviation() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}
