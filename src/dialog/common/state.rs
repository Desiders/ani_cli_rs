pub enum State {
    SelectLanguage,
    SelectSource,
    SelectAnime,
    SelectEpisode,
    SelectQuality,
    LaunchPlayer,
}

impl Default for State {
    #[must_use]
    fn default() -> Self {
        Self::SelectLanguage
    }
}

pub enum ResultState<T> {
    Success(T),
    Break,
}
