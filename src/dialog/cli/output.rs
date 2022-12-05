use std::io::Write as _;
use termcolor::{self, WriteColor as _};

pub fn input_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::White)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn warning_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn error_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn info_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Blue)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn success_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Green)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}

pub fn variant_msg(msg: &str) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout
        .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Cyan)))
        .unwrap();

    write!(stdout, "{}", msg).unwrap();

    stdout.reset().unwrap();
    stdout.flush().unwrap();
}
