use ansi_term::Colour;
use std::io::{stdout, Write};

fn common(msg: &str, with_end: bool, with_tab: bool) {
    if with_end {
        if with_tab {
            println!("\t{}", msg);
        } else {
            println!("{}", msg);
        }
    } else {
        if with_tab {
            print!("\t{}", msg);
        } else {
            print!("{}", msg);
        }
        stdout().flush().unwrap();
    }
}

pub fn input(msg: &str, with_end: bool, with_tab: bool) {
    let decor_msg = format!("{tag} {msg}", tag = Colour::Green.paint(">"), msg = msg);
    common(&decor_msg, with_end, with_tab);
}

pub fn failed(msg: &str, with_end: bool, with_tab: bool) {
    let decor_msg = format!("{tag} {msg}", tag = Colour::Red.paint("<->"), msg = msg);
    common(&decor_msg, with_end, with_tab);
}

pub fn info(msg: &str, with_end: bool, with_tab: bool) {
    let decor_msg = format!("{msg}", msg = Colour::Cyan.paint(msg));
    common(&decor_msg, with_end, with_tab);
}

pub fn variants_info(msg: &str, with_end: bool, with_tab: bool) {
    let decor_msg = format!("{msg}", msg = msg);
    common(&decor_msg, with_end, with_tab);
}
