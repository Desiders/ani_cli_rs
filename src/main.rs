use ani_cli_rs::{dialog, sources::ru::anilibria::source::Anilibria};

fn main() {
    let anilibria = Anilibria::default();

    let sources = vec![&anilibria];

    dialog::run(sources)
}
