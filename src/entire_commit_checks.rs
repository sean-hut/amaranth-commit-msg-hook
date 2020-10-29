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
    fn empty_test() -> Result<(), ()> {
        match empty("") {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }

    #[test]
    fn not_ascii_test() -> Result<(), ()> {
        match not_ascii("❤") {
            Err(_) => Ok(()),
            Ok(_) => Err(()),
        }
    }
}
