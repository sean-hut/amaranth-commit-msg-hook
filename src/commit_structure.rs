use CommitStructure::{
    Invalid, SummaryAndBody, SummaryAndBodyAndFooter, SummaryAndFooter, SummaryOnly,
};

pub enum CommitStructure {
    SummaryOnly,
    SummaryAndBody,
    SummaryAndFooter,
    SummaryAndBodyAndFooter,
    Invalid,
}

pub fn commit_structure(content: &String) -> CommitStructure {
    let third_line: Option<&str> = content.lines().nth(2);

    match third_line {
        None => SummaryOnly,
        Some(_) => {
            if number_of_blank_lines(&content) == 1
                && second_section_is_only_footer_lines(&content) == false
            {
                SummaryAndBody
            } else if number_of_blank_lines(&content) == 1
                && second_section_is_only_footer_lines(&content) == true
            {
                SummaryAndFooter
            } else if number_of_blank_lines(&content) == 2
                && second_section_is_only_footer_lines(&content) == false
                && third_section_is_only_footer_lines(&content) == true
            {
                SummaryAndBodyAndFooter
            } else {
                Invalid
            }
        }
    }
}

fn number_of_blank_lines(content: &String) -> usize {
    content.lines().filter(|x| x.is_empty()).count()
}

fn second_section_is_only_footer_lines(content: &String) -> bool {
    content
        .lines()
        .skip(2)
        .take_while(|x| !x.is_empty())
        .all(|x: &str| x.starts_with("Resolves:") || x.starts_with("See also:"))
}

fn third_section_is_only_footer_lines(content: &String) -> bool {
    content
        .lines()
        .skip(2)
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .take_while(|x| !x.is_empty())
        .all(|x: &str| x.starts_with("Resolves:") || x.starts_with("See also:"))
}
