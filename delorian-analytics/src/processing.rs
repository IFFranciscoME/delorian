//! Sources of data

use regex::Regex;

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

/// grep messages
///
/// From a vector of messages (strings) search for the presence of a query message in a
/// vector of messages to search for. Returns every founded message and index(es) of where it
/// was found.
///
pub fn grep_messages(messages: &[String], query: &[String]) -> Vec<(String, Vec<usize>)> {
    // Compile the regular expressions for each query message
    let query_regexes: Vec<Regex> = query
        .iter()
        .map(|q| Regex::new(&regex::escape(q)).unwrap())
        .collect();

    // Search for each query message in the messages
    let mut search_results: Vec<(String, Vec<usize>)> = Vec::new();

    for (i, message) in messages.iter().enumerate() {
        for (j, regex) in query_regexes.iter().enumerate() {
            if regex.is_match(message) {
                // If the query message is found, add it to the results
                if let Some((_, found_indices)) =
                    search_results.iter_mut().find(|(q, _)| q == &query[j])
                {
                    found_indices.push(i);
                } else {
                    search_results.push((query[j].clone().to_string(), vec![i]));
                }
            }
        }
    }

    search_results
}
