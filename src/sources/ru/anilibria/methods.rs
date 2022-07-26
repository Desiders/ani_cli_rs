use super::{
    api::Api,
    parser::Parser,
    schemas::{anime::Anime, player::Player, playlist::SerieInfo},
    source::Anilibria,
};
use crate::{prompt::process_select, sources::common::methods::Methods};
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
    fn select_serie(&self, schema: &Self::Series) -> Option<Self::Serie> {
        let playlist = &schema.playlist;
        let mut variants: HashMap<&str, &SerieInfo> = HashMap::new();
        for (serie, serie_info) in playlist {
            variants.insert(serie, serie_info);
        }
        let series = &schema.series;
        variants.insert("first", &playlist[&series.first.to_string()]);
        variants.insert("last", &playlist[&series.last.to_string()]);

        process_select(
            "Enter series or any other key to come back: ",
            format!(
                "Info: {}\nFirst episode: {}\nLast episode: {}",
                series.string, series.first, series.last,
            )
            .as_str(),
            variants,
        )
        .map(|s| s.clone())
    }
    fn select_hls(&self, schema: &Self::HlsList) -> Option<Self::Hls> {
        let get_str_by_num = |i: u8| {
            if i == 1 {
                "1"
            } else if i == 2 {
                "2"
            } else {
                "3"
            }
        };
        let mut text = String::new();
        let mut variant_index: u8 = 0;
        let mut variants: HashMap<&str, &String> = HashMap::new();
        if let Some(fhd) = &schema.fhd {
            variant_index += 1;
            let str_variant_index = get_str_by_num(variant_index);
            for variant in [str_variant_index, "fhd", "fullhd", "full hd"] {
                variants.insert(variant, fhd);
            }
            text.push_str(format!("{}. {} (full hd)\n", str_variant_index, "fhd").as_str());
        }
        if let Some(hd) = &schema.hd {
            variant_index += 1;
            let str_variant_index = get_str_by_num(variant_index);
            for variant in [str_variant_index, "hd", "mid", "middle"] {
                variants.insert(variant, hd);
            }
            text.push_str(format!("{}. {}\n", str_variant_index, "hd").as_str());
        }
        if let Some(sd) = &schema.sd {
            variant_index += 1;
            let str_variant_index = get_str_by_num(variant_index);
            for variant in [str_variant_index, "sd", "low"] {
                variants.insert(variant, sd);
            }
            text.push_str(format!("{}. {} (low)\n", str_variant_index, "sd").as_str());
        }
        process_select(
            "Enter hls or any other key to come back: ",
            text.trim(),
            variants,
        )
        .map(|s| s.clone())
    }
    fn get_url(&self, _anime: &Self::Anime, _serie: &Self::Serie, hls: &Self::Hls) -> String {
        let hls = if hls.starts_with("http") {
            hls.to_string()
        } else {
            format!("https://{}", hls)
        };
        format!("{}", hls)
    }
}
