use crate::{
    output::{failed, info},
    prompt::{process_select_variant, read_line_or_none, read_pos_num_or_none},
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
                if let Some((anime, source)) = select_anime(sources) {
                    current_anime = Some(anime);
                    current_source = Some(source);
                    current_state = States::Series;
                } else {
                    current_state = States::Search;
                }
            }
            States::Series => {
                if let Some(serie) = select_serie(
                    current_source.unwrap(),
                    current_anime.as_ref().unwrap(),
                    current_serie.as_ref(),
                ) {
                    current_serie = Some(serie);
                    current_state = States::Hls;
                } else {
                    current_state = States::Search;
                }
            }
            States::Hls => {
                if let Some(hls) =
                    select_hls(current_source.unwrap(), current_serie.as_ref().unwrap())
                {
                    current_hls = Some(hls);
                    current_state = States::Play;
                } else {
                    current_state = States::Series;
                }
            }
            States::Play => {
                play(
                    current_source.unwrap(),
                    current_anime.as_ref().unwrap(),
                    current_serie.as_ref().unwrap(),
                    current_hls.as_ref().unwrap(),
                );

                current_state = States::Series;
                println!();
            }
        }
    }
}

#[must_use]
fn select_anime<'a, Source: Methods + Display>(
    sources: &[&'a Source],
) -> Option<(<Source as Methods>::Anime, &'a Source)> {
    let ani_name = match read_line_or_none("Anime name: ", false) {
        Some(name) => name,
        None => {
            info("You can use Ctrl+C to exit program!", true, false);
            return None;
        }
    };

    let sources_len = sources.len();

    let mut current_source_index: usize = 0;
    let mut current_ani_index: usize = 0;
    let mut ani_common_list = Vec::new();
    let mut ani_source_index_pair = HashMap::new();

    for source in sources {
        println!("{}: ", source);

        match source.search(&ani_name) {
            Err(err) => println!("\t<-> Failed to parse source: {}", err),
            Ok(ani_list) => {
                let ani_list_len = ani_list.len();

                if ani_list_len == 0 {
                    failed("No results", true, true);
                    continue;
                }
                for anime in ani_list {
                    current_ani_index += 1;

                    println!(
                        "\t{index}. {anime}",
                        index = current_ani_index,
                        anime = anime
                    );

                    ani_common_list.push(anime);
                    ani_source_index_pair.insert(current_ani_index, current_source_index);

                    sleep(Duration::from_secs_f32(0.05));
                    if current_ani_index % 10 == 0 && ani_list_len != current_ani_index {
                        match choose_anime_iter_state() {
                            AnimeIterState::Continue => continue,  // continue to next anime
                            AnimeIterState::SwitchSource => break, // switch to next source
                        }
                    }
                }
            }
        }
        if sources_len > 1 {
            match choose_source_iter_state() {
                SourceIterState::Next => {
                    current_source_index += 1;
                    continue; // continue to next source
                }
                SourceIterState::Break => break, // break from loop
            }
        }
    }
    if ani_common_list.is_empty() {
        failed("Anime not found!", true, false);
        return None;
    }
    if let Some(mut index) =
        read_pos_num_or_none("Enter anime index or any other key to come back: ")
    {
        if index == 0 {
            index = 1;
            println!("Hehe, boy");
        }
        if let Some(anime) = ani_common_list.get(index - 1) {
            let anime = anime.clone();
            let source = sources[ani_source_index_pair[&index]];
            return Some((anime, source));
        }
    }
    None
}

#[must_use]
fn select_serie<Source: Methods>(
    source: &Source,
    anime: &<Source as Methods>::Anime,
    current_serie: Option<&<Source as Methods>::Serie>,
) -> Option<<Source as Methods>::Serie> {
    let series = match source.series(anime) {
        Ok(series) => series,
        Err(err) => {
            failed(&format!("Failed to parse series: {}", err), true, false);
            return None;
        }
    };

    let (text, variants) = source.series_info_and_variants(series, current_serie);
    process_select_variant(
        "Enter serie or any other key to come back: ",
        &text,
        &variants,
    )
}

#[must_use]
fn select_hls<Source: Methods>(
    source: &Source,
    serie: &<Source as Methods>::Serie,
) -> Option<<Source as Methods>::Hls> {
    let result = source.hls(serie);
    let hls_list = match result {
        Ok(hls_list) => hls_list,
        Err(err) => {
            failed(&format!("Failed to get hls: {}", err), true, false);
            return None;
        }
    };

    let (text, variants) = source.hls_list_info_and_variants(hls_list);
    process_select_variant(
        "Enter hls or any other key to come back: ",
        &text,
        &variants,
    )
}

fn play<Source: Methods>(
    source: &Source,
    anime: &<Source as Methods>::Anime,
    serie: &<Source as Methods>::Serie,
    hls: &<Source as Methods>::Hls,
) {
    let url = source.get_url(anime, serie, hls);
    let argv = [
        "mpv",
        &url,
        "--fs",
        "--msg-level=all=fatal",
        "--title=Anime",
    ];

    for num in 1..=10 {
        match Popen::create(&argv, PopenConfig::default()) {
            Ok(_) => {
                info("The process launched! Wait opening...", true, false);
                return;
            }
            Err(err) => {
                if num == 10 {
                    failed(
                        &format!(
                            "Couldn't open the MPV player or problems with the anime source! Error: {}.\n
                            Download player and set it in path if you didn't do that before! 
                            Player: https://mpv.io/installation/", err,
                        ),
                        true, false,
                    );
                    return;
                }
                sleep(Duration::from_secs_f32(0.1));
            }
        }
    }
}

#[must_use]
fn choose_anime_iter_state() -> AnimeIterState {
    if let Some(line) = read_line_or_none("Continue parsing this source? (y/other): ", false) {
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
    if let Some(line) =
        read_line_or_none("Continue searching in another source? (y/other): ", false)
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
