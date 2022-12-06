use std::fmt::{self, Display};

use reqwest;
use serde_json;

pub enum SourceError {
    ApiError(String),
    ParseError(String),
    UnknownVariant(String),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(message) => write!(f, "{message}"),
            Self::ParseError(message) => write!(f, "{message}"),
            Self::UnknownVariant(message) => write!(f, "{message}"),
        }
    }
}

impl From<reqwest::Error> for SourceError {
    fn from(error: reqwest::Error) -> Self {
        Self::ApiError(format!("Api error: {error}"))
    }
}

impl From<serde_json::Error> for SourceError {
    fn from(error: serde_json::Error) -> Self {
        Self::ParseError(format!("Parse error: {error}"))
    }
}

impl From<&str> for SourceError {
    fn from(value: &str) -> Self {
        Self::UnknownVariant(format!("Unknown variant: {value}"))
    }
}

pub enum LanguageError {
    UnknownLanguage(String),
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownLanguage(message) => write!(f, "{message}"),
        }
    }
}

pub enum PlayerError {
    UnknownPlayer(String),
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownPlayer(message) => write!(f, "{message}"),
        }
    }
}
