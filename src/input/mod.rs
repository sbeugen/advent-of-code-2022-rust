use std::env;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderMap};

pub fn fetch_input_for_id(id: i32) -> reqwest::Result<String> {
    let mut headers = HeaderMap::new();
    let session_token = env::var("AOC_SESSION_TOKEN").expect("Env variable AOC_SESSION_TOKEN does not exist.");
    headers.insert(COOKIE, format!("session={}", session_token).parse().unwrap());
    let client = Client::new();

    let url = format!("https://adventofcode.com/2022/day/{}/input", id);
    client.get(url).headers(headers).send()?.text()
}