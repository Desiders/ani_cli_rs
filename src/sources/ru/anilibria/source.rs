use reqwest::blocking::Client;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Anilibria {
    name: String,
    language: String,
    base_url: String,
    api_url: String,
    client: reqwest::blocking::Client,
}

impl Anilibria {
    #[must_use]
    pub fn new(
        name: String,
        language: String,
        base_url: String,
        api_url: String,
        client: Client,
    ) -> Anilibria {
        Anilibria {
            name,
            language,
            base_url,
            api_url,
            client,
        }
    }
    #[must_use]
    pub fn default() -> Anilibria {
        Anilibria::new(
            "Anilibria".to_string(),
            "ru".to_string(),
            "https://anilibria.tv".to_string(),
            "https://api.anilibria.tv/v2".to_string(),
            reqwest::blocking::Client::new(),
        )
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[must_use]
    pub fn language(&self) -> &String {
        &self.language
    }
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }
    #[must_use]
    pub fn base_url(&self) -> &String {
        &self.base_url
    }
    #[must_use]
    pub fn api_url(&self) -> &String {
        &self.api_url
    }
    #[must_use]
    pub fn get_info(&self) -> String {
        format!("{} ({})", self.name, self.language)
    }
}

impl Display for Anilibria {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_info())
    }
}
