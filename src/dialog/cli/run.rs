use std::{collections::HashMap, num::IntErrorKind};

use super::{output, prompt};

use crate::{
    dialog::common::{
        state::{ResultState, State},
        state_machine::StateMachine,
    },
    enums::language::Language,
    sources::base::Source,
};

/// Run CLI dialog
#[must_use]
pub fn run<S>(sources: &[S]) -> ResultState<()>
where
    S: Source,
{
    let mut state_machine = StateMachine::default();

    loop {
        match state_machine.current_state() {
            State::SelectLanguage => {
                // Get user input language
                match select_language(sources.iter().map(|source| source.language()).collect()) {
                    ResultState::Success(language) => {
                        // Set input language as current language
                        state_machine.data().set_language(language);
                        // Set next state
                        state_machine.set_state(State::SelectSource);
                    }
                    // Break dialog, because this state is first
                    ResultState::Break => break ResultState::Break,
                }
            }
            State::SelectSource => {
                // Get user input source
                match select_source(sources) {
                    ResultState::Success(source) => {
                        // Set input source as current source
                        state_machine.data().set_source(source.clone());
                        // Set next state
                        state_machine.set_state(State::SelectAnime);
                    }
                    // Go to previous state
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectAnime => {
                let source = state_machine.data().source_mut().unwrap();

                // Select anime
                match select_anime(source) {
                    // Source should save current anime
                    ResultState::Success(_) => {
                        // Set next state
                        state_machine.set_state(State::SelectEpisode);
                    }
                    // Go to previous state
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectEpisode => {
                let source = state_machine.data().source_mut().unwrap();

                // Select episode
                match select_episode(source) {
                    // Source should save current episode
                    ResultState::Success(_) => {
                        // Set next state
                        state_machine.set_state(State::SelectQuality);
                    }
                    // Go to previous state
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectQuality => {
                let source = state_machine.data().source_mut().unwrap();

                // Select quality
                match select_quality(source) {
                    // Source should save current quality
                    ResultState::Success(_) => {
                        // Set next state
                        state_machine.set_state(State::LaunchPlayer);
                    }
                    // Go to previous state
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::LaunchPlayer => todo!(),
        }
    }
}

/// Finish CLI dialog
pub fn finish() {
    output::info_msg("\nBye, peach!\n");
}

/// Select a language
/// # Arguments
/// List of available languages
#[must_use]
fn select_language(sources_languages: Vec<&Language>) -> ResultState<Language> {
    let mut languages: HashMap<&Language, u16> = HashMap::new();
    let mut languages_len = 0;

    for language in sources_languages {
        languages
            .entry(language)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        languages_len += 1;
    }

    let mut languages: Vec<(&Language, u16)> = languages.into_iter().collect();
    languages.sort_by(|(_, a), (_, b)| b.cmp(a));

    output::input_msg("Select a language");
    output::info_msg(" (empty input to back previous state):\n");
    for (seq_num, (language, count)) in languages.iter().enumerate() {
        output::variant_msg(&format!(
            "\t{seq_num}. {language} ({count} sources)\n",
            seq_num = seq_num + 1
        ));
    }

    loop {
        return match prompt::read_line_or_none("\nLanguage: ", None) {
            Some(lang_or_seq_num) => {
                // Check if input is sequence number
                match lang_or_seq_num.parse::<usize>() {
                    // Check if sequence number is valid
                    Ok(seq_num) => {
                        // Check if sequence number is out of range and return first or last language
                        if seq_num <= 0 {
                            break ResultState::Success(languages[0].0.clone());
                        } else if seq_num > languages_len {
                            break ResultState::Success(languages[languages_len - 1].0.clone());
                        // Return language by sequence number if it's valid
                        } else {
                            if let Some((lang, _)) = seq_num
                                .checked_sub(1)
                                .and_then(|seq_num| languages.get(seq_num))
                            {
                                break ResultState::Success((*lang).clone());
                            }
                        }
                    }
                    Err(err) => match err.kind() {
                        IntErrorKind::PosOverflow => {
                            output::warning_msg(&format!(
                                "\nSequence number must be less than {}",
                                usize::MAX
                            ));
                            continue;
                        }
                        IntErrorKind::NegOverflow => {
                            output::warning_msg("\nSequence number must be greater than 0");
                            continue;
                        }
                        IntErrorKind::Empty | IntErrorKind::Zero => unreachable!(),
                        _ => {}
                    },
                }

                match Language::try_from({
                    let lang = lang_or_seq_num;
                    lang
                }) {
                    Ok(lang) => ResultState::Success(lang),
                    Err(err) => {
                        output::warning_msg(&format!("\n{err}"));
                        continue;
                    }
                }
            }
            None => ResultState::Break,
        };
    }
}

/// Select a source
/// # Arguments
/// List of available sources by language
#[must_use]
fn select_source<'a, S>(sources: &'a [S]) -> ResultState<&'a S>
where
    S: Source,
{
    for (seq_num, source) in sources.iter().enumerate() {
        output::variant_msg(&format!("\t{seq_num}. {source}\n", seq_num = seq_num + 1));
    }

    loop {
        return match prompt::read_line_or_none("\nSource: ", None) {
            Some(source_name_or_seq_num) => {
                // Check if input is sequence number
                match source_name_or_seq_num.parse::<usize>() {
                    // Check if sequence number is valid
                    Ok(seq_num) => {
                        // Check if sequence number is out of range and return first or last source
                        if seq_num <= 0 {
                            break ResultState::Success(&sources[0]);
                        } else if seq_num > sources.len() {
                            break ResultState::Success(&sources[sources.len() - 1]);
                        // Return source by sequence number if it's valid
                        } else {
                            if let Some(source) = seq_num
                                .checked_sub(1)
                                .and_then(|seq_num| sources.get(seq_num))
                            {
                                break ResultState::Success(source);
                            }
                        }
                    }
                    Err(err) => match err.kind() {
                        IntErrorKind::PosOverflow => {
                            output::warning_msg(&format!(
                                "\nSequence number must be less than {}",
                                usize::MAX
                            ));
                            continue;
                        }
                        IntErrorKind::NegOverflow => {
                            output::warning_msg("\nSequence number must be greater than 0");
                            continue;
                        }
                        IntErrorKind::Empty | IntErrorKind::Zero => unreachable!(),
                        _ => {}
                    },
                }

                let source_name = source_name_or_seq_num;
                match sources.iter().find(|source| (**source).eq(&source_name)) {
                    Some(source) => ResultState::Success(source),
                    None => {
                        output::warning_msg(&format!("\nSource \"{source_name}\" not found"));
                        continue;
                    }
                }
            }
            None => ResultState::Break,
        };
    }
}

/// Select an anime
/// # Arguments
/// Source to search anime
fn select_anime<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    loop {
        let anime_name = match prompt::read_line_or_none("\nAnime name: ", None) {
            Some(anime_name) => anime_name,
            None => return ResultState::Break,
        };

        let anime_list_info = match source.search_anime_list(&anime_name) {
            Ok(anime_list_info) => anime_list_info,
            Err(err) => {
                output::error_msg(&format!("\n{err}"));
                continue;
            }
        };

        output::variant_msg(&format!(
            "\nAnime list for \"{anime_name}\":\n{anime_list_info}"
        ));

        loop {
            return match prompt::read_line_or_none("\nAnime: ", None) {
                Some(anime_name_or_seq_num) => {
                    match source.select_anime_as_current(anime_name_or_seq_num) {
                        Ok(()) => ResultState::Success(()),
                        Err(err) => {
                            output::warning_msg(&format!("\n{err}"));
                            continue;
                        }
                    }
                }
                None => ResultState::Break,
            };
        }
    }
}

/// Select an episode
/// # Arguments
/// Source to search episode
fn select_episode<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    loop {
        let episode_list_info = match source.episodes_info() {
            Ok(episode_list_info) => episode_list_info,
            Err(err) => {
                output::error_msg(&format!("\n{err}"));
                continue;
            }
        };

        output::variant_msg(&format!("\nEpisode list for anime:\n{episode_list_info}"));

        loop {
            return match prompt::read_line_or_none("\nEpisode: ", None) {
                Some(episode_name_or_seq_num) => {
                    match source.select_episode_as_current(episode_name_or_seq_num) {
                        Ok(()) => ResultState::Success(()),
                        Err(err) => {
                            output::warning_msg(&format!("\n{err}"));
                            continue;
                        }
                    }
                }
                None => ResultState::Break,
            };
        }
    }
}

/// Select a quality
/// # Arguments
/// Source to search quality
fn select_quality<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    loop {
        let quality_list_info = match source.qualities_info() {
            Ok(quality_list_info) => quality_list_info,
            Err(err) => {
                output::error_msg(&format!("\n{err}"));
                continue;
            }
        };

        output::variant_msg(&format!("\nQuality list for episode:\n{quality_list_info}"));

        loop {
            return match prompt::read_line_or_none("\nQuality: ", None) {
                Some(quality_name_or_seq_num) => {
                    match source.select_quality_as_current(quality_name_or_seq_num) {
                        Ok(()) => ResultState::Success(()),
                        Err(err) => {
                            output::warning_msg(&format!("\n{err}"));
                            continue;
                        }
                    }
                }
                None => ResultState::Break,
            };
        }
    }
}
