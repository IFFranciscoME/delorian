//! Sources of data

use regex::Regex;

pub fn grep_messages(log_messages: &[String]) -> Vec<String> {
    // Compile the regular expression to find lines containing "Program ComputeBudget"
    let re = Regex::new(r"Program ComputeBudget").unwrap();

    // Filter the log messages to keep only those that match the pattern
    log_messages
        .iter()
        .filter(|&line| re.is_match(line))
        .cloned()
        .collect()
}
