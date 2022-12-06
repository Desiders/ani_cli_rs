use crate::{
    enums::{language::Language, player::Player},
    sources::base::Source,
};

pub struct Data<S>
where
    S: Source,
{
    language: Language,
    source: Option<S>,
    player: Option<Player>,
}

impl<S> Data<S>
where
    S: Source,
{
    #[must_use]
    pub fn new(language: Language, source: S, player: Player) -> Self {
        Self {
            language,
            source: Some(source),
            player: Some(player),
        }
    }

    #[must_use]
    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    #[must_use]
    pub fn source(&self) -> Option<&S> {
        self.source.as_ref()
    }

    #[must_use]
    pub fn source_mut(&mut self) -> Option<&mut S> {
        self.source.as_mut()
    }

    pub fn set_source(&mut self, source: S) {
        self.source = Some(source);
    }

    #[must_use]
    pub fn player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }
}

impl<S> Default for Data<S>
where
    S: Source,
{
    #[must_use]
    fn default() -> Self {
        Self {
            language: Language::default(),
            source: None,
            player: None,
        }
    }
}
