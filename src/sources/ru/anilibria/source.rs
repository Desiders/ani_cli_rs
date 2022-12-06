use super::{api::Api, parser::Parser, schemas::Anime};
use crate::{enums::language::Language, errors::SourceError, sources::base::Source};

use reqwest;
use std::{
    fmt::{self, Display},
    num::IntErrorKind,
    rc::Rc,
};

#[derive(Clone)]
pub struct Anilibria<'a> {
    name: &'a str,
    language: Language,
    api_url: &'a str,
    client: reqwest::blocking::Client,

    current_anime_list: Vec<Rc<Anime>>,
    current_anime: Option<Rc<Anime>>,
    current_episode: Option<u16>,
    current_quality: Option<String>,
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
            current_quality: None,
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
    type QualitiesInfo = String;
    type QualityInfo = String;

    fn language(&self) -> &Language {
        &self.language
    }

    fn search_anime_list(&mut self, query: &str) -> Result<Self::SearchAnimeListInfo, SourceError> {
        let api_result = <Anilibria as Api>::search_anime(self, query)?;
        let anime_list = <Anilibria as Parser>::search_anime(self, &api_result)?;

        if anime_list.is_empty() {
            return Err(SourceError::ApiError(format!(
                "Anime list by query \"{}\" is empty",
                query
            )));
        }

        let anime_info = anime_list
            .iter()
            .enumerate()
            .map(|(seq_num, anime)| format!("{seq_num}. {anime}\n", seq_num = seq_num + 1))
            .collect();

        self.current_anime_list = anime_list.into_iter().map(Rc::new).collect();

        Ok(anime_info)
    }

    /// # Arguments
    /// Anime title or sequence number in list
    fn select_anime_as_current(&mut self, title_or_seq_num: String) -> Result<(), SourceError> {
        let anime_list = &self.current_anime_list;
        let anime_len = anime_list.len();

        // Check if input is sequence number
        match title_or_seq_num.parse::<usize>() {
            // Check if sequence number is valid
            Ok(seq_num) => {
                // Check if sequence number is out of range and return first or last anime
                if seq_num <= 0 {
                    self.current_anime = Some(Rc::clone(&anime_list[0]));
                    return Ok(());
                } else if seq_num > anime_len {
                    self.current_anime = Some(Rc::clone(&anime_list[anime_len - 1]));
                    return Ok(());
                } else {
                    if let Some(anime) = seq_num
                        .checked_sub(1)
                        .and_then(|seq_num| anime_list.get(seq_num))
                    {
                        self.current_anime = Some(Rc::clone(anime));
                        return Ok(());
                    } else {
                        return Err(SourceError::UnknownVariant(format!(
                            "Unknown anime by sequence number: {seq_num}"
                        )));
                    }
                }
            }
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => {
                    return Err(SourceError::UnknownVariant(format!(
                        "Anime number must be less than {}",
                        usize::MAX
                    )));
                }
                IntErrorKind::NegOverflow => {
                    return Err(SourceError::UnknownVariant(
                        "Anime number must be greater than 0".to_string(),
                    ));
                }
                IntErrorKind::Empty | IntErrorKind::Zero => unreachable!(),
                _ => {}
            },
        }

        let title = title_or_seq_num;
        let anime = anime_list
            .iter()
            .find(|anime| {
                let name = title.to_lowercase();
                let ru_name = anime.names.ru.to_lowercase();
                let en_name = anime.names.en.to_lowercase();

                // Check if anime name is valid
                ru_name == name || en_name == name
            })
            .ok_or(SourceError::UnknownVariant(format!(
                "Unknown anime name: {title}"
            )))?;

        self.current_anime = Some(Rc::clone(anime));

        Ok(())
    }

    fn anime_info(&self) -> Result<Self::AnimeInfo, SourceError> {
        Ok(self
            .current_anime
            .as_ref()
            .map(|anime| format!("{anime}"))
            .expect("No anime selected"))
    }

    fn episodes_info(&mut self) -> Result<Self::EpisodesInfo, SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");

        let episodes_info = format!("\t{series}\n", series = anime.player.series);

        Ok(episodes_info)
    }

    /// # Arguments
    /// Episode number or pattern, e.g. "first", "f", "last", "l"
    fn select_episode_as_current(&mut self, seq_num_or_pattern: String) -> Result<(), SourceError> {
        assert!(!seq_num_or_pattern.is_empty());

        let anime = self.current_anime.as_ref().expect("No anime selected");

        let first = anime.player.series.first;
        let last = anime.player.series.last;

        // Check if input is sequence number
        match seq_num_or_pattern.parse::<u16>() {
            // Check if sequence number is valid
            Ok(seq_num) => {
                // Check if sequence number is out of range and set as current first or last episode
                if seq_num == 0 {
                    self.current_episode = Some(first);
                    return Ok(());
                } else if seq_num >= last {
                    self.current_episode = Some(last);
                    return Ok(());
                } else {
                    // Check if episode number is invalid
                    anime
                        .player
                        .playlist
                        .get(&seq_num_or_pattern.to_lowercase())
                        .ok_or(SourceError::UnknownVariant(format!(
                            "Unknown episode number: {seq_num}"
                        )))?;

                    self.current_episode = Some(seq_num);
                    return Ok(());
                }
            }
            Err(err) => match err.kind() {
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
            },
        }

        let pattern = seq_num_or_pattern;
        let episode = match pattern.to_lowercase().as_str() {
            "first" | "f" => first,
            "last" | "l" => last,
            _ => {
                return Err(SourceError::UnknownVariant(format!(
                    "Unknown episode pattern: {pattern}.\nPossible patterns: first|f, last|l"
                )))
            }
        };

        self.current_episode = Some(episode);

        Ok(())
    }

    fn episode_info(&self) -> Result<Self::EpisodeIndo, SourceError> {
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let episode_info = format!("Episode {episode}\n");

        Ok(episode_info)
    }

    fn qualities_info(&mut self) -> Result<Self::QualitiesInfo, SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_hls_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let mut qualities_info = String::new();

        if serie_with_hls_info.sd.is_some() {
            qualities_info.push_str("sd | 360p | 480p | min | 1\n");
        }
        if serie_with_hls_info.hd.is_some() {
            qualities_info.push_str("hd | 720p | avg | 2\n");
        }
        if serie_with_hls_info.fhd.is_some() {
            qualities_info.push_str("fhd | 1080p | full | max | 3\n");
        }

        Ok(qualities_info)
    }

    /// # Arguments
    /// Quality, e.g. "sd", "hd", "fhd", "full hd", "360p", "480p", "720p", "1080p"
    fn select_quality_as_current(&mut self, quality: String) -> Result<(), SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_quality_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let quality = match quality.to_lowercase().as_str() {
            "sd" | "360p" | "360" | "480p" | "480" | "min" | "1" => {
                serie_with_quality_info.sd.as_ref()
            }
            "hd" | "720p" | "720" | "avg" | "2" => serie_with_quality_info.hd.as_ref(),
            "fhd" | "1080p" | "1080" | "full" | "max" | "3" => serie_with_quality_info.fhd.as_ref(),
            _ => {
                return Err(SourceError::UnknownVariant(format!(
                    "Unknown quality: {}",
                    quality
                )));
            }
        }
        .unwrap()
        .to_owned();

        self.current_quality = Some(quality);

        Ok(())
    }

    fn url_for_stream(&self) -> Result<String, SourceError> {
        let quality = self.current_quality.as_ref().expect("No hls unit selected");

        let url = quality.to_owned();

        Ok(url)
    }
}
