pub enum State {
    SelectLanguage,
    SelectSource,
}

impl Default for State {
    fn default() -> Self {
        Self::SelectLanguage
    }
}

pub enum ResultState<T> {
    Success(T),
    Break,
}
