pub struct Body<'a> {
    body: Result<Vec<&'a str>, &'a str>,
}

pub fn body(content: &str) -> Body {
    Body {
        body: Ok(second_section(&content)),
    }
}

impl<'a> Body<'a> {
    pub fn max_length(&self) -> Result<&'a str, &'a str> {
        match &self.body {
            Ok(s) => match s.iter().map(|x| x.len() > 72).any(|x: bool| x) {
                false => Ok("No body lines longer than 72 chacacters."),
                true => Err("There are body lines longer than 72 chacacters."),
            },
            Err(e) => Err(e),
        }
    }
}

pub fn second_section(content: &str) -> Vec<&str> {
    let section: Vec<&str> = content
        .lines()
        .skip(2)
        .take_while(|x| !x.is_empty())
        .collect();

    section
}

#[cfg(test)]
mod body {

    use super::body;

    #[test]
    fn max_length_test() -> Result<(), ()> {
        let over_max: &str = "D add readme

Readme includes: overview, dependencies, license and semantic verisoning. ";

        match body(&over_max).max_length() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}
