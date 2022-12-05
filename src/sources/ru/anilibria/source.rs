use super::{api::Api, parser::Parser, schemas::Anime};
use crate::{enums::language::Language, errors::SourceError, sources::base::Source};

use reqwest;
use std::{
    fmt::{self, Display},
    num::IntErrorKind,
    rc::Rc,
};

pub struct Anilibria<'a> {
    name: &'a str,
    language: Language,
    api_url: &'a str,
    client: reqwest::blocking::Client,

    current_anime_list: Vec<Rc<Anime>>,
    current_anime: Option<Rc<Anime>>,
    current_episode: Option<u16>,
    current_hls_unit: Option<String>,
}

impl<'a> Anilibria<'a> {
    #[must_use]
    pub fn new(client_builder: Option<reqwest::blocking::ClientBuilder>) -> Self {
        Self {
            name: "Anilibria",
            language: Language::Russian,
            api_url: "https://api.anilibria.tv/v2",
            client: match client_builder {
                Some(builder) => builder.build().unwrap(),
                None => reqwest::blocking::Client::new(),
            },
            current_anime_list: Vec::new(),
            current_anime: None,
            current_episode: None,
            current_hls_unit: None,
        }
    }

    #[must_use]
    pub fn api_url(&self) -> &str {
        self.api_url
    }

    #[must_use]
    pub fn client(&self) -> &reqwest::blocking::Client {
        &self.client
    }
}

impl Default for Anilibria<'_> {
    #[must_use]
    fn default() -> Self {
        Self::new(None)
    }
}

impl Display for Anilibria<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.language)
    }
}

impl PartialEq<String> for Anilibria<'_> {
    fn eq(&self, other: &String) -> bool {
        let other = other.to_lowercase();

        other == "anilibria" || other == "anilibria.tv" || other == "anilib"
    }
}

impl Source for Anilibria<'_> {
    type SearchAnimeListInfo = String;
    type AnimeInfo = String;
    type EpisodesInfo = String;
    type EpisodeIndo = String;
    type HLSListInfo = String;
    type HLSInfo = String;

    fn language(&self) -> &Language {
        &self.language
    }

    fn search_anime(&mut self, query: &str) -> Result<Self::SearchAnimeListInfo, SourceError> {
        let api_result = <Anilibria as Api>::search_anime(self, query)?;
        let anime_list = <Anilibria as Parser>::search_anime(self, &api_result)?;

        let anime_info = anime_list
            .iter()
            .enumerate()
            .map(|(sequence_number, anime)| format!("{}. {}\n", sequence_number + 1, anime))
            .collect();

        self.current_anime_list = anime_list.into_iter().map(Rc::new).collect();

        Ok(anime_info)
    }

    /// # Arguments
    /// Anime name or sequence number in list
    fn select_anime_as_current(&mut self, name_or_seq_num: String) -> Result<(), SourceError> {
        let anime = self
            .current_anime_list
            .iter()
            .find(|anime| {
                let name = name_or_seq_num.to_lowercase();
                let ru_name = anime.names.ru.to_lowercase();
                let en_name = anime.names.en.to_lowercase();

                // Check if anime name is valid
                ru_name == name || en_name == name
            })
            .or_else(|| {
                // Try to parse as sequence number
                let seq_num = name_or_seq_num.parse::<usize>().ok()?;

                // Check if sequence number is valid
                seq_num
                    .checked_sub(1)
                    .and_then(|seq_num| self.current_anime_list.get(seq_num))
            })
            .ok_or_else(|| {
                SourceError::UnknownVariant(format!("Unknown anime: {}", name_or_seq_num))
            })?;

        self.current_anime = Some(Rc::clone(anime));

        Ok(())
    }

    fn anime_info(&self) -> Result<Self::AnimeInfo, SourceError> {
        Ok(self
            .current_anime
            .as_ref()
            .map(|anime| format!("{}", anime))
            .expect("No anime selected"))
    }

    fn episodes_info(&mut self) -> Result<Self::EpisodesInfo, SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");

        let episodes_info = format!("{series}", series = anime.player.series);

        Ok(episodes_info)
    }

    /// # Arguments
    /// Episode number or pattern, e.g. "first", "первая", "last", "последняя", "Фильм"
    fn select_episode_as_current(&mut self, seq_num_or_pattern: String) -> Result<(), SourceError> {
        assert!(!seq_num_or_pattern.is_empty());

        let anime = self.current_anime.as_ref().expect("No anime selected");

        let first = anime.player.series.first;
        let last = anime.player.series.last;

        let episode = match seq_num_or_pattern.parse::<u16>() {
            Ok(seq_num) => {
                if seq_num == 0 {
                    first
                } else if seq_num >= last {
                    last
                } else {
                    if !(first..=last).contains(&seq_num) {
                        return Err(SourceError::UnknownVariant(format!(
                            "Episode number must be in range {first}-{last}, got {seq_num}",
                        )));
                    }

                    seq_num
                }
            }
            Err(err) => {
                match err.kind() {
                    IntErrorKind::PosOverflow => {
                        return Err(SourceError::UnknownVariant(format!(
                            "Episode number must be less than {}",
                            u16::MAX
                        )));
                    }
                    IntErrorKind::NegOverflow => {
                        return Err(SourceError::UnknownVariant(
                            "Episode number must be greater than 0".to_string(),
                        ));
                    }
                    IntErrorKind::Empty | IntErrorKind::Zero => unreachable!(),
                    _ => {}
                }

                let pattern = seq_num_or_pattern.to_lowercase();

                if pattern == "first" || pattern == "первая" || pattern == "f" {
                    first
                } else if pattern == "last" || pattern == "последняя" || pattern == "l" {
                    last
                } else if pattern == "movie" || pattern == "фильм" {
                    first
                } else {
                    return Err(SourceError::UnknownVariant(format!(
                        "Unknown episode pattern: {}",
                        seq_num_or_pattern
                    )));
                }
            }
        };

        self.current_episode = Some(episode);

        Ok(())
    }

    fn episode_info(&self) -> Result<Self::EpisodeIndo, SourceError> {
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let episode_info = format!("Episode {episode}");

        Ok(episode_info)
    }

    fn hls_info(&mut self) -> Result<Self::HLSListInfo, SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_hls_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let mut hls_info = String::new();

        if let Some(ref sd) = serie_with_hls_info.sd {
            hls_info.push_str(&format!("sd: {}\n", sd));
        }
        if let Some(ref hd) = serie_with_hls_info.hd {
            hls_info.push_str(&format!("hd: {}\n", hd));
        }
        if let Some(ref fhd) = serie_with_hls_info.fhd {
            hls_info.push_str(&format!("fhd (full hd): {}\n", fhd));
        }

        Ok(hls_info)
    }

    /// # Arguments
    /// Quality, e.g. "sd", "hd", "fhd", "full hd", "360p", "480p", "720p", "1080p"
    fn select_hls_unit_as_current(&mut self, quality: String) -> Result<(), SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_hls_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let hls_unit = match quality.to_lowercase().as_str() {
            "sd" | "360p" | "360" | "min" => serie_with_hls_info.sd.as_ref(),
            "hd" | "480p" | "480" | "average" | "avg" => serie_with_hls_info.hd.as_ref(),
            "fhd" | "full hd" | "1080p" | "1080" | "full" | "max" => {
                serie_with_hls_info.fhd.as_ref()
            }
            _ => {
                return Err(SourceError::UnknownVariant(format!(
                    "Unknown quality: {}",
                    quality
                )));
            }
        }
        .unwrap()
        .to_owned();

        self.current_hls_unit = Some(hls_unit);

        Ok(())
    }

    fn url_for_stream(&self) -> Result<String, SourceError> {
        let hls_unit = self
            .current_hls_unit
            .as_ref()
            .expect("No hls unit selected");

        let url = hls_unit.to_owned();

        Ok(url)
    }
}
