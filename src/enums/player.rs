use crate::errors::PlayerError;

use std::fmt::{self, Display};

/// Enum for the different players that can be used.
pub enum Player {
    Mpv,
}

impl Player {
    /// Returns a string with all the players that can be used.
    #[must_use]
    pub fn players_info() -> String {
        let players_info = [Player::Mpv]
            .iter()
            .map(|player| format!("\t{player}\n"))
            .collect::<Vec<String>>()
            .join(", ");

        players_info
    }

    /// Returns a string with the documentation for the player.
    #[must_use]
    pub fn player_doc(&self) -> &str {
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

impl TryFrom<String> for Player {
    type Error = PlayerError;

    fn try_from(player: String) -> Result<Self, Self::Error> {
        match player.to_lowercase().as_str() {
            "mpv" => Ok(Self::Mpv),
            _ => Err(PlayerError::UnknownPlayer(format!(
                "Unknown player: {player}"
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
