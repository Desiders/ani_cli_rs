use crate::errors::LanguageError;

use std::fmt::{self, Display};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Language {
    Russian,
    All,
}

impl TryFrom<&str> for Language {
    type Error = LanguageError;

    fn try_from(language: &str) -> Result<Self, Self::Error> {
        match language.to_lowercase().as_str() {
            "russian" | "rus" | "ru" => Ok(Self::Russian),
            "all" => Ok(Self::All),
            _ => Err(LanguageError::UnknownLanguage(format!(
                "Unknown language `{language}`"
            ))),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::All
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Russian => write!(f, "Russian"),
            Language::All => unreachable!(),
        }
    }
}
