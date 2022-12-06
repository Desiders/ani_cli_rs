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

#[allow(clippy::module_name_repetitions)]
pub enum ResultState<T> {
    Success(T),
    Break,
}
