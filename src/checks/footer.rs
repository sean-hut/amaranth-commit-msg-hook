use std::process::exit;

pub struct Footer<'a> {
    footer: Result<Vec<&'a str>, &'a str>,
}

pub fn footer(content: &str, number_blank_lines: usize) -> Footer {
    if number_blank_lines == 1 {
        Footer {
            footer: Ok(second_section(&content)),
        }
    } else if number_blank_lines == 2 {
        Footer {
            footer: Ok(third_section(&content)),
        }
    } else {
        eprintln!("Error: Invalid commit message structure.");
        exit(1);
    }
}

impl<'a> Footer<'a> {
    pub fn max_length(&self) -> Result<&'a str, &'a str> {
        match &self.footer {
            Ok(footer) => match footer.iter().map(|x| x.len() > 72).any(|x: bool| x) {
                false => Ok("No footer lines longer than 72 characters."),
                true => Err("There are footer lines longer than 72 characters."),
            },
            Err(e) => Err(e),
        }
    }

    pub fn all_footer_lines(&self) -> Result<&'a str, &'a str> {
        match &self.footer {
            Ok(footer) => match footer
                .iter()
                .all(|x| x.starts_with("Resolves:") || x.starts_with("See also:"))
            {
                true => Ok("The footer section contains only footer lines."),
                false => Err("The footer section does not contain only footer lines."),
            },
            Err(e) => Err(e),
        }
    }
}

fn second_section(content: &str) -> Vec<&str> {
    let section: Vec<&str> = content
        .lines()
        .skip(2)
        .take_while(|x| !x.is_empty())
        .collect();

    section
}

pub fn third_section(content: &str) -> Vec<&str> {
    let section: Vec<&str> = content
        .lines()
        .skip(2)
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .take_while(|x| !x.is_empty())
        .collect();

    section
}

#[cfg(test)]
mod footer {

    use super::footer;

    fn number_of_blank_lines(content: &str) -> usize {
        content.lines().filter(|x| x.is_empty()).count()
    }

    #[test]
    fn max_length_second_section_test() -> Result<(), ()> {
        let over_max: &str = "R refactor main

    Resolves: ad27df6a5cff694add500ab8c7f97234feb4a91f 898f80736c75878acc02dc55672317fcc0e0a5a6";

        match footer(&over_max, number_of_blank_lines(&over_max)).max_length() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn max_length_third_section_test() -> Result<(), ()> {
        let over_max: &str = "R refactor main

    Change the argument parsing.

    Resolves: ad27df6a5cff694add500ab8c7f97234feb4a91f 898f80736c75878acc02dc55672317fcc0e0a5a6";

        match footer(&over_max, number_of_blank_lines(&over_max)).max_length() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn all_footer_lines_second_section_test() -> Result<(), ()> {
        let footer_lines: &str = "D refactor main

    Resolves: ad27df6a5cff694add500ab8c7f97234feb4a91f";

        match footer(&footer_lines, number_of_blank_lines(&footer_lines)).all_footer_lines() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn all_footer_lines_third_section_test() -> Result<(), ()> {
        let footer_lines: &str = "D refactor main

    Change the argument parsing.

    Resolves: ad27df6a5cff694add500ab8c7f97234feb4a91f";

        match footer(&footer_lines, number_of_blank_lines(&footer_lines)).all_footer_lines() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}
