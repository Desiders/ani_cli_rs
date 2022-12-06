use super::{
    schemas::{Anime, Names, Player, SerieInfo, Series},
    source::Anilibria,
};

use serde_json;

pub trait Parser {
    fn search_anime(&self, query: &str) -> Result<Vec<Anime>, serde_json::Error>;
}

impl Parser for Anilibria<'_> {
    fn search_anime(&self, query: &str) -> Result<Vec<Anime>, serde_json::Error> {
        Ok(serde_json::from_str::<Vec<serde_json::Value>>(query)?
            .iter()
            .map(|value| Anime {
                announce: value["announce"].as_str().map(ToString::to_string),
                names: {
                    let names = value["names"].as_object().unwrap();

                    Names {
                        ru: names["ru"].as_str().unwrap().to_string(),
                        en: names["en"].as_str().unwrap().to_string(),
                    }
                },
                player: Player {
                    host: value["player"]["host"].as_str().unwrap().to_string(),
                    series: {
                        let series = value["player"]["series"].as_object().unwrap();

                        Series {
                            first: series["first"].as_u64().unwrap().try_into().unwrap(),
                            last: series["last"].as_u64().unwrap().try_into().unwrap(),
                            string: series["string"].as_str().unwrap().to_string(),
                        }
                    },
                    playlist: {
                        let player = value["player"].as_object().unwrap();
                        let host = player["host"].as_str().unwrap();

                        player["playlist"]
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| {
                                (k.to_string(), {
                                    let hls = v["hls"].as_object().unwrap();
                                    SerieInfo {
                                        serie: v["serie"].as_u64().unwrap().try_into().unwrap(),
                                        fhd: hls["fhd"].as_str().map(|s| host.to_string() + s),
                                        hd: hls["hd"].as_str().map(|s| host.to_string() + s),
                                        sd: hls["sd"].as_str().map(|s| host.to_string() + s),
                                    }
                                })
                            })
                            .collect()
                    },
                },
            })
            .collect())
    }
}
