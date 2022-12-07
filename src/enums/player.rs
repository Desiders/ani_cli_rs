use crate::errors::PlayerError;

use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Player {
    Mpv,
}

pub const fn players() -> &'static [Player] {
    &[Player::Mpv]
}

impl Player {
    #[must_use]
    pub fn doc(&self) -> &str {
        match self {
            Player::Mpv => {
                "Are you sure you have MPV installed? \
                Try running `mpv --version` in your terminal. \
                If you don't have MPV installed, you can install it from https://mpv.io/installation/ \
                or use another player."
            }
        }
    }
}

impl TryFrom<&str> for Player {
    type Error = PlayerError;

    fn try_from(player: &str) -> Result<Self, Self::Error> {
        match player.to_lowercase().as_str() {
            "mpv" => Ok(Self::Mpv),
            _ => Err(PlayerError::UnknownPlayer(format!(
                "Unknown player `{player}`"
            ))),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Mpv => write!(f, "MPV"),
        }
    }
}
