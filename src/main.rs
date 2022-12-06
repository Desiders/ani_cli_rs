mod dialog;
mod enums;
mod errors;
mod sources;

use enums::language::Language;

fn main() {
    let sources = [sources::ru::anilibria::Anilibria::default()];

    match dialog::cli::run(&sources) {
        dialog::ResultState::Success(_) => unreachable!(),
        dialog::ResultState::Break => {
            dialog::cli::finish();
        }
    }
}
