use crate::enums::language::Language;

#[derive(Default)]
pub struct Data {
    language: Language,
}

impl Data {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}
