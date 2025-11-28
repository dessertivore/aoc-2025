use reqwest;
use std::fs;

pub fn get_aoc_input(year: u32, day: u32) -> String {
    let cookie = fs::read_to_string("cookie.txt")
        .expect("Failed to read cookie.txt")
        .trim()
        .to_string(); // Trim any extra whitespace or newlines

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/{year}/day/{day}/input",
            year = year,
            day = day,
        ))
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .expect("Failed to send request!");

    if response.status().is_success() {
        let text = response.text().expect("Failed to read response text!");
        return text;
    } else {
        panic!("Request failed with status: {}", response.status())
    }
}
