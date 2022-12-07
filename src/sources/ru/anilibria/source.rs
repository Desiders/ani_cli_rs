use super::{api::Api, parser::Parser, schemas::Anime};
use crate::{enums::language::Language, errors::SourceError, sources::base::Source};

use reqwest;
use std::{
    fmt::{self, Display},
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
    current_hls: Option<String>,
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
            current_hls: None,
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
        write!(f, "{}", self.name)
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
                "Anime list by query `{}` is empty",
                query
            )));
        }

        let anime_info = anime_list
            .iter()
            .enumerate()
            .map(|(seq_num, anime)| format!("\t{seq_num}. {anime}\n", seq_num = seq_num + 1))
            .collect();

        self.current_anime_list = anime_list.into_iter().map(Rc::new).collect();

        Ok(anime_info)
    }

    fn select_anime_as_current(&mut self, title_or_seq_num: String) -> Result<(), SourceError> {
        let anime_list = &self.current_anime_list;

        let anime = if let Some(anime) = anime_list.iter().find(|anime| {
            let name = title_or_seq_num.to_lowercase();
            let ru_name = anime.names.ru.to_lowercase();
            let en_name = anime.names.en.to_lowercase();

            // User can different combinations ru and en names,
            // so we use `.contains` for works different situations,
            // e.g `A | B` | `B | A` | `a` (where `A` is `ru`, `B` is `en` and `a` is incomplete name) returns `true`
            ru_name.contains(&name) || en_name.contains(&name)
        }) {
            anime
        } else {
            match title_or_seq_num.parse::<usize>() {
                Ok(seq_num) => {
                    if let Some(anime) = seq_num
                        .checked_sub(1)
                        .and_then(|seq_num| anime_list.get(seq_num))
                    {
                        anime
                    } else {
                        return Err(SourceError::UnknownVariant(format!(
                            "Unknown anime sequence number `{}`",
                            seq_num
                        )));
                    }
                }
                Err(_) => {
                    return Err(SourceError::UnknownVariant(format!(
                        "Unknown anime name `{title_or_seq_num}`"
                    )))
                }
            }
        };

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

        let episodes_info = format!("{episodes}", episodes = anime.player.series);

        Ok(episodes_info)
    }

    fn select_episode_as_current(&mut self, seq_num_or_pattern: String) -> Result<(), SourceError> {
        assert!(!seq_num_or_pattern.is_empty());

        let anime = self.current_anime.as_ref().expect("No anime selected");

        let first = anime.player.series.first;
        let last = anime.player.series.last;

        let episode = match seq_num_or_pattern.to_lowercase().as_str() {
            "first" | "f" => first,
            "last" | "l" => last,
            _ => {
                if let Ok(seq_num) = seq_num_or_pattern.parse::<u16>() {
                    if anime
                        .player
                        .playlist
                        .get(&seq_num_or_pattern.to_lowercase())
                        .is_none()
                    {
                        return Err(SourceError::UnknownVariant(format!(
                            "Unknown episode number `{seq_num}`"
                        )));
                    }
                    seq_num
                } else {
                    return Err(SourceError::UnknownVariant(format!(
                            "Unknown episode pattern `{seq_num_or_pattern}`. Possible patterns: first|f, last|l"
                        )));
                }
            }
        };

        self.current_episode = Some(episode);

        Ok(())
    }

    fn episode_info(&self) -> Result<Self::EpisodeIndo, SourceError> {
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let episode_info = format!("{episode}");

        Ok(episode_info)
    }

    fn qualities_info(&mut self) -> Result<Self::QualitiesInfo, SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_hls_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let mut qualities_info = String::new();

        if serie_with_hls_info.sd.is_some() {
            qualities_info.push_str("\t1 | sd | 360p | 480p | min\n");
        }
        if serie_with_hls_info.hd.is_some() {
            qualities_info.push_str("\t2 | hd | 720p | avg\n");
        }
        if serie_with_hls_info.fhd.is_some() {
            qualities_info.push_str("\t3 | fhd | 1080p | full | max\n");
        }

        Ok(qualities_info)
    }

    fn select_quality_as_current(&mut self, quality: String) -> Result<(), SourceError> {
        let anime = self.current_anime.as_ref().expect("No anime selected");
        let episode = self.current_episode.as_ref().expect("No episode selected");

        let serie_with_hls_info = anime.player.playlist.get(&episode.to_string()).unwrap();

        let hls = match quality.to_lowercase().as_str() {
            "1" | "sd" | "360p" | "360" | "480p" | "480" | "min" => serie_with_hls_info.sd.as_ref(),
            "2" | "hd" | "720p" | "720" | "avg" => serie_with_hls_info.hd.as_ref(),
            "3" | "fhd" | "1080p" | "1080" | "full" | "max" => serie_with_hls_info.fhd.as_ref(),
            _ => {
                return Err(SourceError::UnknownVariant(format!(
                    "Unknown quality `{quality}`"
                )));
            }
        }
        .unwrap();

        self.current_hls = Some(hls.clone());

        Ok(())
    }

    fn url_for_stream(&self) -> Result<String, SourceError> {
        let hls = self.current_hls.as_ref().expect("No hls unit selected");

        if hls.starts_with("http") {
            return Ok(hls.clone());
        }

        let url = format!("https://{hls}");

        Ok(url)
    }
}
