use chrono::{Datelike, Local};
use clap::{value_parser, Arg, Command};
use std::fs::create_dir;

mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_day = Local::now().day().to_string();
    let default_year = Local::now().year().to_string();

    let matches = Command::new("aoc")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .infer_subcommands(true)
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("Aoc allows you to automatize the generation of a new advent of code boilerplate")
        .subcommand(
            Command::new("puzzle")
                .about("introduce the day and the year of the aoc puzzle")
                .arg(
                    Arg::new("day")
                        .long("day")
                        .short('d')
                        .value_parser(value_parser!(String))
                        .help("define the day of the aoc puzzle"),
                )
                .arg(
                    Arg::new("year")
                        .long("year")
                        .short('y')
                        .value_parser(value_parser!(String))
                        .help("define the year of the aoc puzzle"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("puzzle") {
        let day = matches.get_one::<String>("day").unwrap_or(&default_day);
        let year = matches.get_one::<String>("year").unwrap_or(&default_year);

        app::generate_puzzle_app(day, year)?;
    }

    Ok(())
}
