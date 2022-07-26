use crate::{
    prompt::{read_line_or_none, read_pos_num_or_none},
    sources::common::methods::Methods,
};
use std::{collections::HashMap, fmt::Display, thread::sleep, time::Duration};
use subprocess::{Popen, PopenConfig};

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

pub fn run<T>(sources: Vec<&T>)
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
                        println!("You can use Ctrl+C combination to exit program!");
                        continue;
                    }
                };
                let mut source_index = 0;
                let mut anime_index = 0;
                let mut anime_common: Vec<<T as Methods>::Anime> = Vec::new();
                let mut anime_source_index_pair: HashMap<usize, usize> = HashMap::new();

                let sources_len = sources.len();

                for source in &sources {
                    println!("{}:", source);

                    match source.search(&anime_name) {
                        Ok(anime_list) => {
                            let anime_list_len = anime_list.len();

                            if anime_list_len == 0 {
                                println!("\t<-> No results");
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
                if anime_common.len() == 0 {
                    println!("<-> Anime not found by this name!");
                    // current_state = States::Search;
                    // this state is base state
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
                        println!("<-> Failed to get series: {}", err);
                        current_state = States::Search; // set previous state
                        continue;
                    }
                };
                current_serie = match source.select_serie(&series) {
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
                        println!("<-> Failed to get hls: {}", err);
                        current_state = States::Series; // set previous state
                        continue;
                    }
                };
                current_hls = match source.select_hls(&hls_list) {
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

                current_state = States::Hls; // set previous state

                for num in 1..=10 {
                    match Popen::create(
                        &[
                            "mpv",
                            &url,
                            "--fs",
                            "--msg-level=all=fatal",
                            "--title=Anime",
                        ],
                        PopenConfig::default(),
                    ) {
                        Ok(_) => {
                            println!("Successfully started process. You can close terminal!");
                            break;
                        }
                        Err(err) => {
                            if num == 10 {
                                println!("<-> Failed to open url: {}", err);
                                break;
                            }

                            sleep(Duration::from_secs_f32(0.1));
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn choose_anime_iter_state() -> AnimeIterState {
    if let Some(line) = read_line_or_none("Continue parse this source (y/other): ", false) {
        let lower_line = line.to_lowercase();
        let accept_values = ["y", "у", "yes"]; // en "y" and ru "у"
        if accept_values.contains(&lower_line.as_str()) {
            AnimeIterState::Continue
        } else {
            AnimeIterState::SwitchSource
        }
    } else {
        AnimeIterState::SwitchSource
    }
}

fn choose_source_iter_state() -> SourceIterState {
    if let Some(line) = read_line_or_none(
        "Continue searching anime in other sources (y/other): ",
        false,
    ) {
        if line.to_lowercase() == "y" {
            SourceIterState::Next
        } else {
            SourceIterState::Break
        }
    } else {
        SourceIterState::Break
    }
}
