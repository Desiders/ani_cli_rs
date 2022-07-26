use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

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

pub fn read_pos_num_or_none(input_msg: &str) -> Option<usize> {
    if let Some(line) = read_line_or_none(input_msg, false) {
        match line.parse::<usize>() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("\t<-> Failed to parse positive number!");
                None
            }
        }
    } else {
        None
    }
}

pub fn process_select<'a, T>(
    input_msg: &str,
    text: &str,
    variants: HashMap<&str, &'a T>,
) -> Option<&'a T> {
    println!("{}", text);
    if let Some(line) = read_line_or_none(input_msg, false) {
        variants.get(line.to_lowercase().as_str()).map(|v| *v)
    } else {
        None
    }
}
