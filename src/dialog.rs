use crate::{
    output::{failed, info},
    prompt::{process_select_variant, read_line_or_none, read_pos_num_or_none},
    sources::common::methods::Methods,
};
use std::{collections::HashMap, fmt::Display, thread::sleep, time::Duration};
use subprocess::{Popen, PopenConfig, Redirection};

enum States {
    Search,
    Series,
    Hls,
    Play,
}

enum AnimeIterState {
    Continue,
    SwitchSource,
}

enum SourceIterState {
    Next,
    Break,
}

pub fn run<T>(sources: &[&T])
where
    T: Methods + Display,
{
    let mut current_state = States::Search;
    let mut current_source: Option<&T> = None;
    let mut current_anime: Option<<T as Methods>::Anime> = None;
    let mut current_serie: Option<<T as Methods>::Serie> = None;
    let mut current_hls: Option<<T as Methods>::Hls> = None;

    loop {
        match current_state {
            States::Search => {
                let anime_name = match read_line_or_none("Anime name: ", false) {
                    Some(line) => line,
                    None => {
                        info(
                            "You can use Ctrl+C combination to exit program!",
                            true,
                            false,
                        );
                        continue;
                    }
                };
                let mut source_index = 0;
                let mut anime_index = 0;
                let mut anime_common: Vec<<T as Methods>::Anime> = Vec::new();
                let mut anime_source_index_pair: HashMap<usize, usize> = HashMap::new();

                let sources_len = sources.len();

                for source in sources {
                    println!("{}:", source);

                    match source.search(&anime_name) {
                        Ok(anime_list) => {
                            let anime_list_len = anime_list.len();

                            if anime_list_len == 0 {
                                failed("No results", true, true);
                            }
                            for anime in anime_list {
                                anime_index += 1;
                                println!("\t{}. {}", anime_index, anime);
                                anime_common.push(anime);
                                anime_source_index_pair.insert(anime_index, source_index);

                                if anime_index % 10 == 0 && anime_list_len != anime_index {
                                    match choose_anime_iter_state() {
                                        AnimeIterState::Continue => continue,  // continue to next anime
                                        AnimeIterState::SwitchSource => break, // switch to next source
                                    }
                                }
                                sleep(Duration::from_secs_f32(0.05));
                            }
                        }
                        Err(err) => println!("\t<-> Failed to parse source: {}", err),
                    }
                    if sources_len > 1 {
                        match choose_source_iter_state() {
                            SourceIterState::Next => {
                                source_index += 1;
                                continue;
                            } // continue to next source
                            SourceIterState::Break => break, // break from loop
                        }
                    }
                }
                if anime_common.is_empty() {
                    failed("Anime not found by this name!", true, false);
                    current_state = States::Search; // this state is base state
                    continue;
                }
                if let Some(mut index) =
                    read_pos_num_or_none("Enter anime index or any other key to come back: ")
                {
                    if index == 0 {
                        index = 1;
                        println!("Hehe, boy");
                    }
                    if let Some(anime) = anime_common.get(index - 1) {
                        current_source = Some(sources[anime_source_index_pair[&index]]);
                        current_anime = Some(anime.clone());
                        current_state = States::Series; // set next state
                    } else {
                        current_state = States::Search; // back to base state
                    }
                }
            }
            States::Series => {
                let source = current_source.unwrap();
                let anime = current_anime.as_ref().unwrap();
                let series = match source.series(anime) {
                    Ok(series) => series,
                    Err(err) => {
                        failed(&format!("Failed to parse series: {}", err), true, false);
                        current_state = States::Search; // set previous state
                        continue;
                    }
                };

                let (text, variants) =
                    source.series_info_and_variants(series, current_serie.as_ref());
                current_serie = match process_select_variant(
                    "Enter series, some abbreviation, or any other key to come back: ",
                    &text,
                    variants,
                ) {
                    Some(serie) => {
                        current_state = States::Hls; // set next state
                        Some(serie)
                    }
                    None => {
                        current_state = States::Search; // set previous state
                        continue;
                    }
                };
            }
            States::Hls => {
                let source = current_source.unwrap();
                let serie = current_serie.as_ref().unwrap();

                let result = source.hls(serie);
                let hls_list = match result {
                    Ok(hls_list) => hls_list,
                    Err(err) => {
                        failed(&format!("Failed to get hls: {}", err), true, false);
                        current_state = States::Series; // set previous state
                        continue;
                    }
                };

                let (text, variants) = source.hls_list_info_and_variants(hls_list);
                current_hls = match process_select_variant(
                    "Enter hls or any other key to come back: ",
                    &text,
                    variants,
                ) {
                    Some(hls) => {
                        current_state = States::Play; // set next state
                        Some(hls)
                    }
                    None => {
                        current_state = States::Series; // set previous state
                        continue;
                    }
                };
            }
            States::Play => {
                let source = current_source.unwrap();
                let anime = current_anime.as_ref().unwrap();
                let serie = current_serie.as_ref().unwrap();
                let hls = current_hls.as_ref().unwrap();

                let url = source.get_url(anime, serie, hls);

                for num in 1..=10 {
                    match Popen::create(
                        &[
                            "mpv",
                            &url,
                            "--fs",
                            "--msg-level=all=fatal",
                            "--title=Anime",
                        ],
                        PopenConfig {
                            stdin: Redirection::None,
                            stdout: Redirection::None,
                            stderr: Redirection::None,
                            detached: true,
                            ..Default::default()
                        },
                    ) {
                        Ok(_) => {
                            current_state = States::Series; // set series state
                            current_hls = None; // reset HLS state, because no need it
                            info(
                                "The process successfully launched! Wait opening...",
                                true,
                                false,
                            );
                            break;
                        }
                        Err(err) => {
                            if num == 10 {
                                failed(
                                    &format!(
                                        "<-> Couldn't open the MPV player or problems with the anime source! Error: {}.\n
                                        Download player and set it in path if you didn't do that before! 
                                        Player: https://mpv.io/installation/", err,
                                    ),
                                    true, false,
                                );
                                current_state = States::Hls; // set previous state
                                break;
                            }

                            sleep(Duration::from_secs_f32(0.1));
                            continue;
                        }
                    }
                }
                println!();
            }
        }
    }
}

#[must_use]
fn choose_anime_iter_state() -> AnimeIterState {
    if let Some(line) = read_line_or_none("Continue parsing the source? (y/other): ", false) {
        let lower_line = line.to_lowercase();
        let accept_values = ["y", "у"]; // en "y" and ru "у"
        if accept_values.contains(&lower_line.as_str()) {
            AnimeIterState::Continue
        } else {
            AnimeIterState::SwitchSource
        }
    } else {
        AnimeIterState::SwitchSource
    }
}

#[must_use]
fn choose_source_iter_state() -> SourceIterState {
    if let Some(line) = read_line_or_none("Continue searching in another source? (y/other):", false)
    {
        let lower_line = line.to_lowercase();
        let accept_values = ["y", "у"]; // en "y" and ru "у"
        if accept_values.contains(&lower_line.as_str()) {
            SourceIterState::Next
        } else {
            SourceIterState::Break
        }
    } else {
        SourceIterState::Break
    }
}
