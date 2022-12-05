use crate::{enums::language::Language, sources::base::Source};

pub struct Data<'a, S>
where
    S: Source,
{
    language: Language,
    source: Option<&'a S>,
}

impl<'a, S> Data<'a, S>
where
    S: Source,
{
    #[must_use]
    pub fn new(language: Language, source: &'a S) -> Self {
        Self {
            language,
            source: Some(source),
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
    pub fn source(&self) -> Option<&'a S> {
        self.source.map(|source| source)
    }

    pub fn set_source(&mut self, source: &'a S) {
        self.source = Some(source);
    }
}

impl<S> Default for Data<'_, S>
where
    S: Source,
{
    fn default() -> Self {
        Self {
            language: Language::default(),
            source: None,
        }
    }
}
