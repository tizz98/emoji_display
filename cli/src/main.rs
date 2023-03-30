use clap::{App, Arg};
use emoji_core::lookup_emoji;
use std::process;

fn main() {
    let matches = App::new("Emoji CLI")
        .version("0.1.0")
        .author("Elijah Wilson <dev.tizz98@gmail.com>")
        .about("Lookup emojis by keyword")
        .arg(
            Arg::new("KEYWORD")
                .help("The keyword for the emoji")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(keyword) = matches.value_of("KEYWORD") {
        match lookup_emoji(keyword) {
            Some(emoji) => println!("{}", emoji),
            None => {
                eprintln!("No emoji found for keyword: {}", keyword);
                process::exit(1);
            }
        }
    }
}
