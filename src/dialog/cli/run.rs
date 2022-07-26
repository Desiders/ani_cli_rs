use super::{output, prompt};

use crate::{
    dialog::common::{
        state::{ResultState, State},
        state_machine::StateMachine,
    },
    enums::{
        language::Language,
        player::{players, Player},
    },
    players::mpv,
    sources::base::Source,
};

use std::collections::HashMap;

pub fn run<S>(sources: &[S])
where
    S: Source,
{
    let mut state_machine = StateMachine::default();

    loop {
        match state_machine.current_state() {
            State::SelectLanguage => {
                match select_language(sources.iter().map(Source::language).collect()) {
                    ResultState::Success(language) => {
                        state_machine.data().set_language(language);
                        state_machine.set_state(State::SelectSource);
                    }
                    ResultState::Break => break,
                }
            }
            State::SelectSource => {
                let language = state_machine.data().language();
                match select_source(
                    &sources
                        .iter()
                        .filter(|source| source.language().eq(language))
                        .collect::<Vec<&S>>(),
                ) {
                    ResultState::Success(source) => {
                        state_machine.data().set_source(source.clone());
                        state_machine.set_state(State::SelectAnime);
                    }
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectAnime => {
                let source = state_machine.data().source_mut().unwrap();

                match select_anime(source) {
                    ResultState::Success(_) => {
                        let anime_info = source.anime_info().expect("Anime isn't set");
                        output::info_msg(&format!("\tSelected anime `{anime_info}`\n"));

                        state_machine.set_state(State::SelectEpisode);
                    }
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectEpisode => {
                let source = state_machine.data().source_mut().unwrap();

                match select_episode(source) {
                    ResultState::Success(_) => {
                        let episode_info = source.episode_info().expect("Episode isn't set");
                        output::info_msg(&format!("\tSelected episode `{episode_info}`\n"));

                        state_machine.set_state(State::SelectQuality);
                    }
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectQuality => {
                let source = state_machine.data().source_mut().unwrap();

                match select_quality(source) {
                    ResultState::Success(_) => {
                        state_machine.set_state(State::SelectPlayer);
                    }
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
            State::SelectPlayer => match select_player() {
                ResultState::Success(player) => {
                    state_machine.data().set_player(player);
                    state_machine.set_state(State::LaunchPlayer);
                }
                ResultState::Break => state_machine.set_previous_state(),
            },
            State::LaunchPlayer => {
                let data = state_machine.data();

                let player = data.player().unwrap().clone();
                let source = data.source_mut().unwrap();

                match launch_player(source, &player) {
                    ResultState::Success(_) => match select_state() {
                        ResultState::Success(State::SelectAnime) => {
                            state_machine.set_previous_state_and_truncate_next(State::SelectAnime);
                        }
                        ResultState::Success(State::SelectEpisode) => {
                            state_machine
                                .set_previous_state_and_truncate_next(State::SelectEpisode);
                        }
                        ResultState::Success(State::SelectQuality) => {
                            state_machine
                                .set_previous_state_and_truncate_next(State::SelectQuality);
                        }
                        ResultState::Success(_) => unreachable!(),
                        ResultState::Break => state_machine.set_previous_state(),
                    },
                    ResultState::Break => state_machine.set_previous_state(),
                }
            }
        }
        println!();
    }

    finish();
}

pub fn finish() {
    output::info_msg("\nBye, peach!\n");
}

#[must_use]
fn select_language(sources_languages: Vec<&Language>) -> ResultState<Language> {
    let mut languages: HashMap<&Language, u16> = HashMap::new();

    for language in sources_languages {
        languages
            .entry(language)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut languages: Vec<(&Language, u16)> = languages.into_iter().collect();
    languages.sort_by(|(_, a), (_, b)| b.cmp(a));

    output::variant_headline_msg("Available languages");
    output::info_msg(" (enter empty input to back previous state):\n");

    for (seq_num, (language, count)) in languages.iter().enumerate() {
        output::variant_msg(&format!(
            "\t{seq_num}. {language} ({count} sources)\n",
            seq_num = seq_num + 1
        ));
    }

    loop {
        return match prompt::read_line_or_none("Select a language: ", None) {
            Some(lang_or_seq_num) => match Language::try_from(lang_or_seq_num.as_str()) {
                Ok(language) => ResultState::Success(language),
                Err(err) => {
                    if let Ok(seq_num) = lang_or_seq_num.parse::<usize>() {
                        if let Some((language, _)) = seq_num
                            .checked_sub(1)
                            .and_then(|seq_num| languages.get(seq_num))
                        {
                            ResultState::Success((*language).clone())
                        } else {
                            output::warning_msg(&format!(
                                "Unknown language sequence number `{seq_num}`\n"
                            ));
                            continue;
                        }
                    } else {
                        output::warning_msg(&format!("{err}\n"));
                        continue;
                    }
                }
            },
            None => ResultState::Break,
        };
    }
}

#[must_use]
fn select_source<'a, S>(sources: &[&'a S]) -> ResultState<&'a S>
where
    S: Source,
{
    output::variant_headline_msg("Available sources:\n");

    for (seq_num, source) in sources.iter().enumerate() {
        output::variant_msg(&format!("\t{seq_num}. {source}\n", seq_num = seq_num + 1));
    }

    loop {
        return match prompt::read_line_or_none("Select a source: ", None) {
            Some(source_name_or_seq_num) => {
                if let Some(source) = sources
                    .iter()
                    .find(|source| (**source).eq(&source_name_or_seq_num))
                {
                    ResultState::Success(source)
                } else if let Ok(seq_num) = source_name_or_seq_num.parse::<usize>() {
                    if let Some(source) = seq_num
                        .checked_sub(1)
                        .and_then(|seq_num| sources.get(seq_num))
                    {
                        ResultState::Success(source)
                    } else {
                        output::warning_msg(&format!(
                            "Unknown source sequence number `{seq_num}`\n"
                        ));
                        continue;
                    }
                } else {
                    output::warning_msg(&format!("Unknown source `{source_name_or_seq_num}`\n"));
                    continue;
                }
            }
            None => ResultState::Break,
        };
    }
}

fn select_anime<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    loop {
        let anime_name = match prompt::read_line_or_none("Enter anime name: ", None) {
            Some(anime_name) => anime_name,
            None => return ResultState::Break,
        };

        let anime_list_info = match source.search_anime_list(&anime_name) {
            Ok(anime_list_info) => anime_list_info,
            Err(err) => {
                output::error_msg(&format!("{err}\n"));
                continue;
            }
        };

        output::variant_headline_msg(&format!("Anime list:\n{anime_list_info}"));

        loop {
            return match prompt::read_line_or_none("Select anime: ", None) {
                Some(anime_name_or_seq_num) => {
                    if let Err(err) = source.select_anime_as_current(anime_name_or_seq_num) {
                        output::warning_msg(&format!("{err}\n"));
                        continue;
                    }

                    ResultState::Success(())
                }
                None => ResultState::Break,
            };
        }
    }
}

fn select_episode<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    let episode_list_info = match source.episodes_info() {
        Ok(episode_list_info) => episode_list_info,
        Err(err) => {
            output::error_msg(&format!("{err}"));
            return ResultState::Break;
        }
    };

    output::variant_headline_msg(&format!("Episodes: {episode_list_info}"));

    loop {
        return match prompt::read_line_or_none("Select an episode: ", None) {
            Some(episode_name_or_seq_num) => {
                if let Err(err) = source.select_episode_as_current(episode_name_or_seq_num) {
                    output::warning_msg(&format!("{err}\n"));
                    continue;
                }

                ResultState::Success(())
            }
            None => ResultState::Break,
        };
    }
}

fn select_quality<S>(source: &mut S) -> ResultState<()>
where
    S: Source,
{
    let quality_list_info = match source.qualities_info() {
        Ok(quality_list_info) => quality_list_info,
        Err(err) => {
            output::error_msg(&format!("{err}"));
            return ResultState::Break;
        }
    };

    output::variant_headline_msg(&format!("Qualities:\n{quality_list_info}"));

    loop {
        return match prompt::read_line_or_none("Select a quality: ", None) {
            Some(quality_name_or_seq_num) => {
                if let Err(err) = source.select_quality_as_current(quality_name_or_seq_num) {
                    output::warning_msg(&format!("{err}\n"));
                    continue;
                }

                ResultState::Success(())
            }
            None => ResultState::Break,
        };
    }
}

fn select_player() -> ResultState<Player> {
    let players = players();

    output::variant_headline_msg("Available players:\n");

    for (seq_num, player) in players.iter().enumerate() {
        output::variant_msg(&format!("\t{seq_num}. {player}\n", seq_num = seq_num + 1));
    }

    loop {
        let player = match prompt::read_line_or_none("Select a player: ", None) {
            Some(player_name_or_seq_num) => match Player::try_from(player_name_or_seq_num.as_str())
            {
                Ok(player) => player,
                Err(err) => {
                    if let Ok(seq_num) = player_name_or_seq_num.parse::<usize>() {
                        if let Some(player) = seq_num
                            .checked_sub(1)
                            .and_then(|seq_num| players.get(seq_num))
                        {
                            player.clone()
                        } else {
                            output::warning_msg(&format!(
                                "Unknown player sequence number `{seq_num}`\n"
                            ));
                            continue;
                        }
                    } else {
                        output::warning_msg(&format!("{err}\n"));
                        continue;
                    }
                }
            },
            None => return ResultState::Break,
        };

        match player {
            Player::Mpv => {
                if mpv::is_installed() {
                    return ResultState::Success(player);
                }
            }
        }

        output::error_msg(&format!("{}\n", player.doc()));
    }
}

fn launch_player<S>(source: &mut S, player: &Player) -> ResultState<()>
where
    S: Source,
{
    let url = match source.url_for_stream() {
        Ok(url) => url,
        Err(err) => {
            output::error_msg(&format!("{err}"));
            return ResultState::Break;
        }
    };

    output::info_msg("Launch the process! Wait opening...\n");

    match player {
        Player::Mpv => {
            if let Err(err) = mpv::launch(&url) {
                output::error_msg(&format!("{err}"));
                return ResultState::Break;
            }
        }
    }

    output::info_msg("Process finished!\n\n");

    ResultState::Success(())
}

fn select_state() -> ResultState<State> {
    let states = [
        State::SelectAnime,
        State::SelectEpisode,
        State::SelectQuality,
    ];

    output::variant_headline_msg("What do you want to do next?");
    output::info_msg(" (enter empty input to back previous state)\n");

    for (seq_num, state) in states.iter().enumerate() {
        output::variant_msg(&format!("\t{seq_num}. {state}\n", seq_num = seq_num + 1));
    }

    loop {
        let state = match prompt::read_line_or_none("Select a state: ", None) {
            Some(state_name_or_seq_num) => match State::try_from(state_name_or_seq_num.as_str()) {
                Ok(state) => state,
                Err(err) => {
                    if let Ok(seq_num) = state_name_or_seq_num.parse::<usize>() {
                        if let Some(state) = seq_num
                            .checked_sub(1)
                            .and_then(|seq_num| states.get(seq_num))
                        {
                            state.clone()
                        } else {
                            output::warning_msg(&format!(
                                "Unknown state sequence number `{seq_num}`\n"
                            ));
                            continue;
                        }
                    } else {
                        output::warning_msg(&format!("{err}\n"));
                        continue;
                    }
                }
            },
            None => return ResultState::Break,
        };

        return ResultState::Success(state);
    }
}
