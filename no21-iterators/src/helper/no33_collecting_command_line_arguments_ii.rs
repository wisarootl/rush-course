#![allow(unused)]

use std::env;
use std::process;

#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}

pub fn main() {
    println!("=== no33_collecting_command_line_arguments_ii ===");
    let settings = collect_settings();
    println!("{settings:?}");
}

fn collect_settings() -> Option<Settings> {
    /*
    ```
    cargo run -- rust.mp4 true false nonsense
    ```

    output
    ```
    Settings { video_file: "rust.mp4", subtitles: true, high_definition: false }
    ```
    */

    let mut args = env::args().skip(1).take(3);

    // let video_file = args.next().unwrap_or_else(|| {
    //     eprintln!("No video file specified!");
    //     process::exit(1);
    // });

    let video_file = args.next()?;

    let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

    let subtitles = settings.next().unwrap_or(false);
    let high_definition = settings.next().unwrap_or(false);

    Some(Settings {
        video_file,
        subtitles,
        high_definition,
    })
}
