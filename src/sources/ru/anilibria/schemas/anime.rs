use super::{names::Names, player::Player};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Anime {
    pub announce: Option<String>, // "Серии выходят каждое воскресенье"
    pub names: Names,
    pub player: Player,
}

impl Display for Anime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} | {}", self.names.ru, self.names.en)
    }
}
