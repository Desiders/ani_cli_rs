use super::{
    api::Api,
    parser::Parser,
    schemas::{anime::Anime, player::Player, playlist::SerieInfo},
    source::Anilibria,
};
use crate::sources::common::methods::Methods;
use reqwest::Result as ReqwestResult;
use std::collections::HashMap;

impl Methods for Anilibria {
    type Anime = Anime;
    type Series = Player;
    type Serie = SerieInfo;
    type HlsList = SerieInfo;
    type Hls = String;

    fn search(&self, query: &str) -> ReqwestResult<Vec<Self::Anime>> {
        Ok(Parser::search(self, &Api::search(self, query)?))
    }
    fn series(&self, schema: &Self::Anime) -> ReqwestResult<Self::Series> {
        Ok(schema.player.clone())
    }
    fn hls(&self, schema: &Self::Serie) -> ReqwestResult<Self::HlsList> {
        Ok(schema.clone())
    }
    fn series_info_and_variants(
        &self,
        schema: Self::Series,
        serie: Option<&Self::Serie>,
    ) -> (String, HashMap<String, Self::Serie>) {
        let mut variants = HashMap::new();
        let playlist = schema.playlist;
        for (serie, serie_info) in &playlist {
            variants.insert(serie.clone(), serie_info.clone());
        }
        let series = schema.series;
        let first_serie_string = series.first.to_string();
        let last_serie_string = series.last.to_string();
        variants.insert("first".to_string(), playlist[&first_serie_string].clone());
        variants.insert("last".to_string(), playlist[&last_serie_string].clone());
        let mut text = String::new();
        text.push_str(
            format!(
                "Info: {}\nFirst episode: {}\nLast episode: {}",
                series.string, series.first, series.last,
            )
            .as_str(),
        );
        if let Some(serie_info) = serie {
            let serie_previous = serie_info.serie - 1;
            let serie_next = serie_info.serie + 1;
            if let Some(serie_info) = playlist.get(&serie_previous.to_string()) {
                let previous_variants = ["p", "previous", "-"];
                for variant in previous_variants {
                    variants.insert(variant.to_string(), serie_info.clone());
                }
                text.push_str(
                    format!(
                        "\nPrevious episode: {} ({})",
                        serie_previous,
                        previous_variants.join(" | ")
                    )
                    .as_str(),
                );
            }
            if let Some(serie_info) = playlist.get(&serie_next.to_string()) {
                let next_variants = ["n", "next", "+"];
                for variant in next_variants {
                    variants.insert(variant.to_string(), serie_info.clone());
                }
                text.push_str(
                    format!(
                        "\nNext episode: {} ({})",
                        serie_next,
                        next_variants.join(" | ")
                    )
                    .as_str(),
                );
            }
        }
        (text, variants)
    }
    fn hls_list_info_and_variants(
        &self,
        schema: Self::HlsList,
    ) -> (String, HashMap<String, Self::Hls>) {
        let mut variants = HashMap::new();
        let mut text = String::new();
        let mut variant_index: u8 = 0;
        for (result, string) in [
            (schema.fhd, "full hd"),
            (schema.hd, "hd"),
            (schema.sd, "sd"),
        ] {
            if let Some(hls) = result {
                variant_index += 1;
                variants.insert(variant_index.to_string(), hls);
                text.push_str(format!("{}. {}\n", variant_index, string).as_str());
            }
        }
        (text.trim().to_string(), variants)
    }
    fn get_url(&self, _anime: &Self::Anime, _serie: &Self::Serie, hls: &Self::Hls) -> String {
        if hls.starts_with("http") {
            hls.to_string()
        } else {
            format!("https://{}", hls)
        }
    }
}
