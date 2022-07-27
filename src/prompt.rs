use std::{
    collections::HashMap,
    hash::BuildHasher,
    io::{stdin, stdout, Write},
};

#[must_use]
pub fn read_line_or_none(input_msg: &str, can_empty: bool) -> Option<String> {
    let mut line = String::new();

    print!("\n> {}", input_msg);
    stdout().flush().unwrap();

    match stdin().read_line(&mut line) {
        Ok(_) => {
            if !can_empty && line.trim().is_empty() {
                println!("\t<-> Empty string!");
                None
            } else {
                Some(line.trim().to_string())
            }
        }
        Err(err) => {
            println!("\t<-> Failed to read line: {}", err);
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
            println!("\t<-> Failed to parse positive number!");
            None
        }
    } else {
        None
    }
}

#[must_use]
pub fn process_select_variant<T, S: BuildHasher>(
    input_msg: &str,
    text: &str,
    mut variants: HashMap<String, T, S>, // `mut` for `.remove()` for a possible to move without Copy
) -> Option<T> {
    println!("{}", text);
    if let Some(line) = read_line_or_none(input_msg, false) {
        variants.remove(line.to_lowercase().as_str())
    } else {
        None
    }
}
