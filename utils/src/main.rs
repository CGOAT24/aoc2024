use std::fs::File;
use std::io::Write;
use chrono::{Datelike, Local};
use reqwest::header::{HeaderValue, COOKIE};

async fn get_input(aoc_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let today = Local::now();
    let day = today.day();
    let year = today.year();

    let session_header = format!("session={}", aoc_token);

    let endpoint = format!("https://adventofcode.com/{year}/day/{day}/input");
    let directory = format!("../day{:02}", day);
    let file_path = format!("{}/input.txt", directory);

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session_header.as_str())?);

    let request = client.get(endpoint).headers(headers).send().await?;
    let response = request.text().await?;

    if !std::path::Path::new(directory.as_str()).exists() {
        std::fs::create_dir_all(&directory)?;
    }
    let mut file = File::create(file_path)?;
    file.write_all(response.as_ref())?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("Missing AOC token");
        return;
    }

    let token = &args[1];
    if let Err(err) = get_input(token).await {
        eprintln!("{}", err);
    }
    else {
        println!("Done!");
    }
}

