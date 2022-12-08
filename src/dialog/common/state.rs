use crate::errors::StateError;

use std::fmt::{self, Display};

#[derive(Clone, Eq, PartialEq)]
pub enum State {
    SelectLanguage,
    SelectSource,
    SelectAnime,
    SelectEpisode,
    SelectQuality,
    SelectPlayer,
    LaunchPlayer,
}

impl Default for State {
    #[must_use]
    fn default() -> Self {
        Self::SelectLanguage
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelectLanguage => write!(f, "Select Language"),
            Self::SelectSource => write!(f, "Select Source"),
            Self::SelectAnime => write!(f, "Select Anime"),
            Self::SelectEpisode => write!(f, "Select Episode"),
            Self::SelectQuality => write!(f, "Select Quality"),
            Self::SelectPlayer => write!(f, "Select Player"),
            Self::LaunchPlayer => write!(f, "Launch Player"),
        }
    }
}

impl TryFrom<&str> for State {
    type Error = StateError;

    fn try_from(state: &str) -> Result<Self, Self::Error> {
        match state.to_lowercase().as_str() {
            "language" | "lang" => Ok(Self::SelectLanguage),
            "source" => Ok(Self::SelectSource),
            "anime" => Ok(Self::SelectAnime),
            "episode" => Ok(Self::SelectEpisode),
            "quality" => Ok(Self::SelectQuality),
            "player" => Ok(Self::SelectPlayer),
            "launch" => Ok(Self::LaunchPlayer),
            _ => Err(StateError::UnknownState(format!("Unknown state `{state}`"))),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub enum ResultState<T> {
    Success(T),
    Break,
}
