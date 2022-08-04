use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn input(msg: &str, with_end: bool, with_tab: bool) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    if with_tab {
        write!(&mut buffer, "\t").unwrap();
    }
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    write!(&mut buffer, ">").unwrap();
    buffer.reset().unwrap();
    write!(&mut buffer, " {}", msg).unwrap();
    if with_end {
        writeln!(&mut buffer).unwrap();
    }
    bufwtr.print(&buffer).unwrap();
}

pub fn failed(msg: &str, with_end: bool, with_tab: bool) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    if with_tab {
        write!(&mut buffer, "\t").unwrap();
    }
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(&mut buffer, "<->").unwrap();
    buffer.reset().unwrap();
    write!(&mut buffer, " {}", msg).unwrap();
    if with_end {
        writeln!(&mut buffer).unwrap();
    }
    bufwtr.print(&buffer).unwrap();
}

pub fn info(msg: &str, with_end: bool, with_tab: bool) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    if with_tab {
        write!(&mut buffer, "\t").unwrap();
    }
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    write!(&mut buffer, "{}", msg).unwrap();
    if with_end {
        buffer.reset().unwrap();
        writeln!(&mut buffer).unwrap();
    }
    bufwtr.print(&buffer).unwrap();
}

pub fn variants_info(msg: &str, with_end: bool, with_tab: bool) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    if with_tab {
        write!(&mut buffer, "\t").unwrap();
    }
    write!(&mut buffer, "{}", msg).unwrap();
    if with_end {
        writeln!(&mut buffer).unwrap();
    }
    bufwtr.print(&buffer).unwrap();
}
