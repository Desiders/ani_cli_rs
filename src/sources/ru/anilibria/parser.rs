use super::schemas::{
    anime::Anime, names::Names, player::Player, playlist::SerieInfo, series::Series,
};
use super::source::Anilibria;
use serde_json::Value;

pub trait Parser {
    fn search(&self, text: &str) -> Vec<Anime>;
}

impl Parser for Anilibria {
    fn search(&self, text: &str) -> Vec<Anime> {
        let mut anime_list = Vec::new();
        for value in serde_json::from_str::<Vec<Value>>(text).unwrap() {
            anime_list.push(Anime {
                announce: value["announce"].as_str().map(|s| s.to_string()),
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
                            first: series["first"].as_u64().unwrap() as u16,
                            last: series["last"].as_u64().unwrap() as u16,
                            string: series["string"].as_str().unwrap().to_string(),
                        }
                    },
                    playlist: {
                        let player = value["player"].as_object().unwrap();
                        player["playlist"]
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| {
                                (k.to_string(), {
                                    let hls = v["hls"].as_object().unwrap();
                                    let host = player["host"].as_str().unwrap();
                                    SerieInfo {
                                        serie: v["serie"].as_u64().unwrap() as u16,
                                        fhd: hls["fhd"].as_str().map(|s| host.to_string() + s),
                                        hd: hls["hd"].as_str().map(|s| host.to_string() + s),
                                        sd: hls["sd"].as_str().map(|s| host.to_string() + s),
                                    }
                                })
                            })
                            .collect()
                    },
                },
            });
        }
        anime_list
    }
}
