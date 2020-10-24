pub mod body_checks;
pub mod entire_commit_checks;
pub mod summary_checks;

use crate::entire_commit_checks::{empty, not_ascii};

use crate::summary_checks::{
    first_summary_word_not_imperative_mood, first_summary_word_not_lowercase,
    invalid_category_abbreviation, summary_ends_with_period, summary_over_50_characters,
};

use crate::body_checks::lines_over_max_length;

pub fn check_commit_message(content: &String) {
    let checks = vec![
        // entire commit message
        empty(&content),
        not_ascii(&content),
        // summary checks
        invalid_category_abbreviation(&content),
        summary_ends_with_period(&content),
        summary_over_50_characters(&content),
        first_summary_word_not_lowercase(&content),
        first_summary_word_not_imperative_mood(&content),
        // body checks
        lines_over_max_length(&content),
    ];

    ()
}
