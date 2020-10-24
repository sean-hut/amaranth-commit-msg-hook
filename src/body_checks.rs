pub fn lines_over_max_length(content: &String) -> Result<String, String> {
    match content.lines().skip(2).next() {
        None => Ok("No body lines longer than 72 chacacters.".to_string()),
        Some(_) => match check_body_lines(&content) {
            false => Ok("No body lines longer than 72 chacacters.".to_string()),
            true => Err("There are body lines longer than 72 chacacters.".to_string()),
        },
    }
}

fn check_body_lines(content: &String) -> bool {
    content
        .lines()
        .skip(2)
        .map(|x| x.len() > 72)
        .any(|x: bool| x == true)
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
