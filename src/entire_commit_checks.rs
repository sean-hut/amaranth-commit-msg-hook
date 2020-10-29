pub fn empty(content: &str) -> Result<&str, &str> {
    match content.is_empty() {
        false => Ok("Commit message is not empty."),
        true => Err("Commit message is empty."),
    }
}

pub fn not_ascii(content: &str) -> Result<&str, &str> {
    match content.is_ascii() {
        true => Ok("Commit message is ASCII."),
        false => Err("Commit message is not ASCII."),
    }
}

#[cfg(test)]
mod entire_commit {

    use super::{empty, not_ascii};

    #[test]
    fn empty_test() -> Result<(), String> {
        let empty_message: String = "".to_string();

        match empty(&empty_message) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected.".to_string()),
        }
    }

    #[test]
    fn not_ascii_test() -> Result<(), String> {
        let not_ascii_message: String = "❤".to_string();

        match not_ascii(&not_ascii_message) {
            Err(_) => Ok(()),
            Ok(_) => Err("Did not error as expected.".to_string()),
        }
    }
}
