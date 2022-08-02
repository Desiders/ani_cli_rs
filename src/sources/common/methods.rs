use reqwest::Result as ReqwestResult;
use std::{collections::HashMap, fmt::Display};

pub trait Methods {
    type Anime: Display + Clone;
    type Series;
    type Serie: Clone;
    type HlsList;
    type Hls: Clone;

    fn search(&self, query: &str) -> ReqwestResult<Vec<Self::Anime>>;
    fn series(&self, schema: &Self::Anime) -> ReqwestResult<Self::Series>;
    fn hls(&self, schema: &Self::Serie) -> ReqwestResult<Self::HlsList>;

    fn series_info_and_variants(
        &self,
        schema: Self::Series,
        serie: Option<&Self::Serie>,
    ) -> (String, HashMap<String, Self::Serie>);
    fn hls_list_info_and_variants(
        &self,
        schema: Self::HlsList,
    ) -> (String, HashMap<String, Self::Hls>);

    fn get_url(&self, anime: &Self::Anime, serie: &Self::Serie, hls: &Self::Hls) -> String;
}
