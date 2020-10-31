pub struct SignOff<'a> {
    sign_off_line: Result<&'a str, &'a str>,
}

impl<'a> SignOff<'a> {
    pub fn sign_off(content: &str, number_blank_lines: usize) -> SignOff {
        let section = if number_blank_lines == 2 {
            content.lines().nth(2)
        } else if number_blank_lines == 3 {
            content.lines().skip(2).skip_while(|x| !x.is_empty()).nth(1)
        } else if number_blank_lines == 4 {
            content
                .lines()
                .skip(2)
                .skip_while(|x| !x.is_empty())
                .skip(1)
                .skip_while(|x| !x.is_empty())
                .nth(1)
        } else {
            None
        };

        SignOff {
            sign_off_line: match section {
                Some(s) => Ok(s),
                None => Err("Problem reading commit message sign off line."),
            },
        }
    }

    pub fn sign_off_line(&self) -> Result<&'a str, &'a str> {
        match &self.sign_off_line {
            Ok(s) => match s.starts_with("Signed-off-by:") {
                true => Ok("There is a sign off line."),
                false => Err("There is not a sign off line."),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod sign_off {

    use super::SignOff;

    #[test]
    fn sign_off_line_test() -> Result<(), ()> {
        let sign_off: &str = "R refactor main\
                              \n\
                              Add verbose command line option.\
                              \n\
                              <NAME> <EMAIL>\
                              \n";

        match SignOff::sign_off(&sign_off, 3).sign_off_line() {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}
