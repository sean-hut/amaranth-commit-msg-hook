pub fn lines_over_max_length(content: &str) -> Result<String, String> {
    match content.lines().nth(2) {
        None => Ok("No body lines longer than 72 chacacters.".to_string()),
        Some(_) => match check_body_lines(&content) {
            false => Ok("No body lines longer than 72 chacacters.".to_string()),
            true => Err("There are body lines longer than 72 chacacters.".to_string()),
        },
pub struct Body<'a> {
    body: Result<Vec<&'a str>, &'a str>,
}

pub fn body(content: &str) -> Body {
    Body {
        body: Ok(second_section(&content)),
    }
}

fn check_body_lines(content: &str) -> bool {
    content
        .lines()
        .skip(2)
        .map(|x| x.len() > 72)
        .any(|x: bool| x)
}

#[cfg(test)]
mod body {

    use super::lines_over_max_length;

    #[test]
    fn lines_over_max_length_test() -> Result<(), String> {
        let line_over_max: String = "D add readme

Readme includes: overview, dependencies, license and semantic verisoning. "
            .to_string();

        match lines_over_max_length(&line_over_max) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected.".to_string()),
        }
    }
}
