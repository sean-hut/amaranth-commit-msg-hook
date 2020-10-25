pub fn empty(content: &String) -> Result<String, String> {
    match content.is_empty() {
        false => Ok("Commit message is not empty.".to_string()),
        true => Err("Commit message is empty.".to_string()),
    }
}

pub fn not_ascii(content: &String) -> Result<String, String> {
    match content.is_ascii() {
        true => Ok("Commit message is ASCII.".to_string()),
        false => Err("Commit message is not ASCII.".to_string()),
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
