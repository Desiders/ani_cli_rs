use super::{Names, Player};

use std::fmt::{self, Display};

pub struct Anime {
    pub announce: Option<String>, // "Серии выходят каждое воскресенье"
    pub names: Names,
    pub player: Player,
}

impl Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{names}", names = self.names)?;

        if let Some(announce) = &self.announce {
            write!(f, " | {announce}", announce = announce)?;
        }

        Ok(())
    }
}
