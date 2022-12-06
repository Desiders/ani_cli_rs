use std::fmt::Display;

use crate::{errors::SourceError, Language};

/// A base trait for all sources
/// # Sub traits
/// - [Display](std::fmt::Display): Display the name and language of the source, e.g. `Anilibria (Russian)`
/// - [PartialEq](std::cmp::PartialEq): Compare source by name,
///     e.g. `Anilibria` || `anilibria` || `anilibria.tv` for [Anilibria](crate::sources::ru::anilibria::Anilibria)
pub trait Source: Clone + Display + PartialEq<String> {
    type SearchAnimeListInfo: Display;
    type AnimeInfo: Display;
    type EpisodesInfo: Display;
    type EpisodeIndo: Display;
    type QualitiesInfo: Display;
    type QualityInfo: Display;

    /// Get language of the source
    fn language(&self) -> &Language;

    /// Search anime by name
    /// # Arguments
    /// * `name` - Anime name.
    fn search_anime_list(&mut self, query: &str) -> Result<Self::SearchAnimeListInfo, SourceError>;

    /// Select an anime as current anime. \
    /// This method is used to select an anime from list of anime
    /// and select it for future use. \
    /// Source should remember selected the anime and use it in other related methods.
    /// # Arguments
    /// Any detail that can define an anime. \
    /// For example, anime name, sequence number in list and other. \
    /// What is the best way to specify the anime, depends on the source.
    fn select_anime_as_current(&mut self, _: String) -> Result<(), SourceError>;

    /// Get information about the anime
    fn anime_info(&self) -> Result<Self::AnimeInfo, SourceError>;

    /// Get information about episodes of current anime
    fn episodes_info(&mut self) -> Result<Self::EpisodesInfo, SourceError>;

    /// Select an episode as current episode. \
    /// This method is used to select an episode from list of episodes
    /// and select it for future use. \
    /// Source should remember selected the episode and use it in other related methods.
    /// # Arguments
    /// Any detail that can define an episode. \
    /// For example, episode name, sequence number in list and other. \
    /// What is the best way to specify the episode, depends on the source.
    fn select_episode_as_current(&mut self, _: String) -> Result<(), SourceError>;

    /// Get information about the episode
    fn episode_info(&self) -> Result<Self::EpisodeIndo, SourceError>;

    /// Get information about qualities of current episode
    fn qualities_info(&mut self) -> Result<Self::QualitiesInfo, SourceError>;

    /// Select a quality as current quality. \
    /// This method is used to select a quality from qualities
    /// and select it for future use. \
    /// Source should remember selected the quality and use it in other related methods.
    /// # Arguments
    /// Any detail that can define a quality. \
    /// For example, quality name, sequence number in list and other. \
    /// What is the best way to specify the quality, depends on the source.
    fn select_quality_as_current(&mut self, _: String) -> Result<(), SourceError>;

    /// Get url for steam anime and use it in player
    fn url_for_stream(&self) -> Result<String, SourceError>;
}
