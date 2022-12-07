mod dialog;
mod enums;
mod errors;
mod players;
mod sources;

use enums::language::Language;

fn main() {
    let sources = [sources::ru::anilibria::Anilibria::default()];

    dialog::cli::run(&sources);
}
