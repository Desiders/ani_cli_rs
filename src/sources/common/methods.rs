use reqwest::Result as ReqwestResult;
use std::fmt::Display;

pub trait Methods {
    type Anime: Display + Clone;
    type Series;
    type Serie;
    type HlsList;
    type Hls;

    fn search(&self, query: &str) -> ReqwestResult<Vec<Self::Anime>>;
    fn series(&self, schema: &Self::Anime) -> ReqwestResult<Self::Series>;
    fn hls(&self, schema: &Self::Serie) -> ReqwestResult<Self::HlsList>;

    fn select_serie(&self, schema: &Self::Series) -> Option<Self::Serie>;
    fn select_hls(&self, schema: &Self::HlsList) -> Option<Self::Hls>;

    fn get_url(&self, anime: &Self::Anime, serie: &Self::Serie, hls: &Self::Hls) -> String;
}
