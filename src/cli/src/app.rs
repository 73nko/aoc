use std::fs::{read_to_string, OpenOptions};
use std::process::Command;

use std::io::Error;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use chrono::{FixedOffset, NaiveDate, TimeZone, Utc};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE};
use reqwest::redirect::Policy;

const SESSION_COOKIE_FILE: &str = ".adventofcode.session";
const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;
const DECEMBER: u32 = 12;

fn generate_template() -> String {
    r#"
use anyhow::Result;

fn part1(input: &Vec<&str>) -> Result<usize> {
    Ok(0)
}
fn part2(input: &Vec<&str>) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 0);
    }
}

fn main() -> Result<()> {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("PART 1: {:?}", part1(&input)?);
    // println!("PART 2: {:?}", part2(&input)?);
    Ok(())
}
    "#
    .to_string()
}

fn read_session_cookie() -> String {
    let path = PathBuf::from(SESSION_COOKIE_FILE);

    match read_to_string(&path) {
        Ok(cookie) => cookie,
        Err(err) => {
            eprintln!(
                "error: Failed to read session cookie from \"{}\": {}",
                path.display(),
                err
            );
            exit(2);
        }
    }
}

fn puzzle_unlocked(day: &str, year: &str) -> Result<bool, String> {
    let timezone = FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET).unwrap();
    let now = timezone.from_utc_datetime(&Utc::now().naive_utc());
    let puzzle_date =
        NaiveDate::from_ymd_opt(year.parse().unwrap(), DECEMBER, day.parse().unwrap())
            .ok_or_else(|| format!("Invalid date: day {}, year {}.", day, year))?
            .and_hms_opt(0, 0, 0)
            .unwrap();
    let unlock_time = timezone.from_local_datetime(&puzzle_date).single();

    if let Some(time) = unlock_time {
        Ok(now.signed_duration_since(time).num_milliseconds() >= 0)
    } else {
        Ok(false)
    }
}

fn build_client(session_cookie: &str, content_type: &str) -> Result<Client, String> {
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))
        .map_err(|err| format!("Invalid session cookie: {}", err))?;
    let content_type_header = HeaderValue::from_str(content_type).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_type_header);

    Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .map_err(|err| err.to_string())
}

fn download_input(day: &str, year: &str) -> Result<String, String> {
    let session = read_session_cookie();
    let formatted_day = day.parse::<i32>().unwrap();
    if !puzzle_unlocked(day, year)? {
        return Err(format!("Puzzle {} of {} is still locked.", day, year));
    }

    eprintln!("Downloading input for day {}, {}...", day, year);

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, formatted_day
    );
    let content_type = "text/plain";
    let puzzle_input = build_client(&session, content_type)?
        .get(&url)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())
        .map_err(|err| err.to_string())?;

    Ok(puzzle_input)
}

pub fn generate_puzzle_app(day: &str, year: &str) -> Result<(), Error> {
    let puzzle_app = format!("puzzle_{}_{}", year, day);
    let project_path = format!("./puzzles/{}", puzzle_app);
    let main_file = format!("{}/src/main.rs", project_path);
    let input_file = format!("{}/input.txt", project_path);
    let cargo_file = format!("{}/Cargo.toml", project_path);
    let template = generate_template();

    Command::new("cargo")
        .arg("new")
        .arg(&project_path)
        .output()
        .expect("failed to create new app");

    // Create new puzzle of the day file and fill it with the template
    let mut puzzle_file = OpenOptions::new()
        .write(true)
        .open(&main_file)
        .expect("Unable to open add file");
    puzzle_file.write_all(template.as_bytes())?;

    // Create and download the input file
    let puzzle_input = download_input(day, year).unwrap();
    let mut puzzle_input_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&input_file)
        .expect("Error creating input file");

    puzzle_input_file.write_all(puzzle_input.as_bytes())?;

    // Add anyhow dependency
    let mut cargo_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&cargo_file)
        .expect("Error creating input file");

    cargo_file.write_all("anyhow={ version = \"1.0.66\"}".as_bytes())?;

    Ok(())
}
