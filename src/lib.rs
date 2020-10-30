pub mod checks;

use std::process::exit;

use crate::checks::{
    body::{body, second_section},
    entire::{empty, not_ascii},
    footer::{footer, third_section},
    summary::summary,
};

pub fn check_commit_message(content: &str) {
    output_check_results(&check_results(&content));

    program_exit(&check_results(&content));
}

}

fn output_check_results(checks: &[Result<String, String>]) {
    for x in checks {
        match x {
            Ok(pass_message) => println!("[PASS] {}", pass_message),
            Err(fail_message) => eprintln!("[FAIL] {}", fail_message),
        }
    }
}

fn any_failed_checks(checks: &[Result<String, String>]) -> bool {
    checks.iter().map(|x| x.is_err()).any(|x| x)
}

fn program_exit(checks: &[Result<String, String>]) {
    match any_failed_checks(&checks) {
        true => {
            eprintln!(
                "Error: The commit message failed to pass all the Amaranth commit message checks."
            );
            exit(1);
        }
        false => {
            println!("[Success] The commit message passed all the Amaranth commit message checks.")
        }
    }
}
