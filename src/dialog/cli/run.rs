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
pub fn run<S>(sources: &[&mut S]) -> ResultState<()>
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
                    ResultState::Break => return ResultState::Break,
                }
            }
            State::SelectSource => {
                // Get user input source
                match select_source(sources.iter().map(|source| &**source).collect()) {
                    ResultState::Success(source) => {
                        // Set input source as current source
                        state_machine.data().set_source(source);
                        // Set next state
                        state_machine.set_state(State::SelectAnime);
                    }
                    // Go to previous state
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectAnime => todo!(),
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
    output::info_msg(" (empty input to exit):\n");
    for (index, (language, count)) in languages.iter().enumerate() {
        output::variant_msg(&format!(
            "\t{}. {} ({} sources)\n",
            index + 1,
            language,
            count
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
                        output::warning_msg(&format!("\n{}", err));
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
fn select_source<'a, S>(sources: Vec<&'a S>) -> ResultState<&'a S>
where
    S: Source,
{
    for (index, source) in sources.iter().enumerate() {
        output::variant_msg(&format!("\t{}. {}\n", index + 1, source));
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
                            break ResultState::Success(sources[0]);
                        } else if seq_num > sources.len() {
                            break ResultState::Success(sources[sources.len() - 1]);
                        // Return source by sequence number if it's valid
                        } else {
                            if let Some(source) = seq_num
                                .checked_sub(1)
                                .and_then(|seq_num| sources.get(seq_num))
                            {
                                break ResultState::Success(*source);
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
                    Some(source) => ResultState::Success(*source),
                    None => {
                        output::warning_msg(&format!("\nSource \"{}\" not found", source_name));
                        continue;
                    }
                }
            }
            None => ResultState::Break,
        };
    }
}
