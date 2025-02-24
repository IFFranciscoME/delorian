//! Sources of data

// use regex::Regex;

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

pub fn grep_messages(log_messages: &[String]) -> bool {
    // Compile the regular expression to find lines containing "Program ComputeBudget"
    let some_pattern = String::from("ComputeBudget111111111111111111111111111111");

    // Filter the log messages to keep only those that match the pattern
    let presence: bool = if log_messages.iter().any(|n| n == &some_pattern) {
        true
    } else {
        false
    };
    presence
}
