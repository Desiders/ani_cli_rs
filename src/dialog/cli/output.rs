use std::io::Write as _;
use termcolor::{self, WriteColor as _};

pub fn input_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Green)))
        .unwrap();

    let msg = if msg.starts_with('\n') {
        writeln!(stdout).unwrap();
        msg.replacen('\n', "", 1)
    } else {
        msg.to_string()
    };

    write!(stdout, "> ").unwrap();
    stdout.reset().unwrap();
    write!(stdout, "{msg}").unwrap();

    stdout.flush().unwrap();
}

pub fn warning_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
        .unwrap();

    let msg = if msg.starts_with('\n') {
        writeln!(stdout).unwrap();
        msg.replacen('\n', "", 1)
    } else {
        msg.to_string()
    };

    write!(stdout, "\t<-> ").unwrap();
    stdout.reset().unwrap();
    write!(stdout, "{msg}").unwrap();
}

pub fn error_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
        .unwrap();

    let msg = if msg.starts_with('\n') {
        writeln!(stdout).unwrap();
        msg.replacen('\n', "", 1)
    } else {
        msg.to_string()
    };

    write!(stdout, "\t<-> ").unwrap();
    stdout.reset().unwrap();
    write!(stdout, "{msg}").unwrap();
}

pub fn info_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Blue)))
        .unwrap();

    write!(stdout, "{msg}").unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn variant_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::White)))
        .unwrap();

    write!(stdout, "{msg}").unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn variant_headline_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Cyan)))
        .unwrap();

    let msg = if msg.starts_with('\n') {
        writeln!(stdout).unwrap();
        msg.replacen('\n', "", 1)
    } else {
        msg.to_string()
    };

    write!(stdout, "> ").unwrap();
    stdout.reset().unwrap();
    write!(stdout, "{msg}").unwrap();

    stdout.flush().unwrap();
}
