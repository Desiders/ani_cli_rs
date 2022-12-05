use super::source::Anilibria;

use reqwest;

pub trait Api {
    fn search_anime(&self, query: &str) -> Result<String, reqwest::Error>;
}

impl Api for Anilibria<'_> {
    fn search_anime(&self, query: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/searchTitles", self.api_url());

        self.client()
            .get(&url)
            .query(&[("search", query), ("limit", "30")])
            .send()?
            .text()
    }
}
