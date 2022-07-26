use reqwest::Result as ReqwestResult;

use super::source::Anilibria;

pub trait Api {
    fn search(&self, query: &str) -> ReqwestResult<String>;
}

impl Api for Anilibria {
    fn search(&self, query: &str) -> ReqwestResult<String> {
        self.client()
            .get(format!(
                "{}/searchTitles?search={}&limit=30",
                self.api_url(),
                query
            ))
            .send()?
            .text()
    }
}
