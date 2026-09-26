//! Video Player Application
//! Command-Line Arguments:
//!
//! - Video File Name
//! - Subtitles
//! - High Definition
use std::env;
use std::process;

#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}
fn main() {
    let settings = collect_setting();
    println!("{:?}", settings);
}
fn collect_setting() -> Settings {
    let args: Vec<String> = env::args().skip(1).take(3).collect();
    if args.len() < 1 {
        panic!(
            "Usage: {} <video_file> [subtitles] [high_definition]",
            args[0]
        );
    }
    let video_file = args
        .iter()
        .next()
        .unwrap_or_else(|| {
            eprintln!("error: no `<video_file>` supplied");
            process::exit(1);
        })
        .clone();
    //let subtitles = args.contains(&String::from("subtitles"));
    //let high_definition = args.contains(&String::from("high_definition"));

    let settings = args
        .iter()
        .skip(1)
        .map(|setting| setting.parse::<bool>().unwrap_or(false))
        .collect::<Vec<bool>>();
    let subtitles = settings.iter().next().cloned().unwrap_or(false);
    let high_definition = settings.iter().nth(1).cloned().unwrap_or(false);
    Settings {
        video_file,
        subtitles,
        high_definition,
    }
}
