use super::output;

use std::io;

/// Reads a line from stdin
/// # Arguments
/// * `empty_warning_msg` - A message to print if the input is empty, `None` if no message should be printed
/// # Returns
/// * `Some(String)` - The trimmed input
/// * `None` - If the input is empty or an error occurred
#[must_use]
pub fn read_line_or_none(input_msg: &str, empty_warning_msg: Option<&str>) -> Option<String> {
    let mut input = String::new();

    // Print the input message
    output::input_msg(input_msg);

    // Read a line from stdin
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim();

            // If the input is empty
            if trimmed_input.is_empty() {
                // If a warning message should be printed
                if let Some(msg) = empty_warning_msg {
                    // Print the warning message
                    output::warning_msg(msg);
                }
                return None;
            }

            // Return the trimmed input
            Some(trimmed_input.to_string())
        }
        Err(_) => None,
    }
}
