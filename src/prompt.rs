use crate::output::{failed, input, variants_info};
use std::{collections::HashMap, hash::BuildHasher, io::stdin};

#[must_use]
pub fn read_line_or_none(input_msg: &str, can_empty: bool) -> Option<String> {
    let mut line = String::new();

    input(input_msg, false, false);
    match stdin().read_line(&mut line) {
        Ok(_) => {
            if !can_empty && line.trim().is_empty() {
                failed("Empty line is not allowed!", true, true);
                None
            } else {
                Some(line.trim().to_string())
            }
        }
        Err(err) => {
            failed(&format!("Error while reading line: {}", err), true, true);
            None
        }
    }
}

#[must_use]
pub fn read_pos_num_or_none(input_msg: &str) -> Option<usize> {
    if let Some(line) = read_line_or_none(input_msg, false) {
        if let Ok(num) = line.parse::<usize>() {
            Some(num)
        } else {
            failed("Invalid positive number!", true, true);
            None
        }
    } else {
        None
    }
}

#[must_use]
fn select_variant<T: Clone, S: BuildHasher>(
    variant: &str,
    variants: &HashMap<String, T, S>,
) -> Option<T> {
    variants
        .get(variant.to_lowercase().as_str())
        .map(|obj| obj.clone())
}

#[must_use]
pub fn process_select_variant<T: Clone, S: BuildHasher>(
    input_msg: &str,
    text: &str,
    variants: &HashMap<String, T, S>,
) -> Option<T> {
    variants_info(text, true, false);
    if let Some(variant) = read_line_or_none(input_msg, false) {
        select_variant(&variant, variants)
    } else {
        None
    }
}
