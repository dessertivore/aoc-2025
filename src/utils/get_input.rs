use reqwest;
use std::fs;

/// Fetches the Advent of Code input for a given year and day.
///
/// # Parameters
///
/// * `year` - The year of the Advent of Code event.
/// * `day` - The day of the Advent of Code event.
///
/// # Returns
///
/// A `String` containing the input for the specified year and day.
///
/// # Panics
///
/// This function will panic in the following scenarios:
/// - If the `testing` configuration is enabled and the year is not 2025.
/// - If the test input file for the specified day cannot be read.
/// - If the `cookie.txt` file cannot be read.
/// - If the HTTP request to fetch the input fails.
/// - If the HTTP response status is not successful.
///
/// # Behavior
///
/// - In testing mode (`cfg!(test)`), the function reads the input from a local file
///   located in the `test-inputs` directory. The file name is expected to follow the
///   format `day_<day>.txt`. Files used for testing this func should have an impossible
///   date number (i.e. > 31).
/// - In non-testing mode, the function reads the session cookie from `cookie.txt`
///   and uses it to authenticate an HTTP request to the Advent of Code website.
///   The input is fetched from the URL `https://adventofcode.com/{year}/day/{day}/input`.
///
/// # Example
///
/// ```rust
/// use aoc_2025::utils::get_input;
///
/// #[cfg(test)]
/// let input = get_input::get_aoc_input(2025, 1);
/// ```
pub fn get_aoc_input(year: u32, day: u32) -> String {
    let testing: bool = cfg!(test);
    if testing {
        if year != 2025 {
            panic!("Only 2025 test inputs can be found here.")
        }
        let file_name: String = format!("test-inputs/day_{}.txt", day);
        let text = fs::read_to_string(file_name).expect("Unable to read text file :(");
        return text;
    }
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
        response.text().expect("Failed to read response text!")
    } else {
        panic!("Request failed with status: {}", response.status())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_aoc_test_input() {
        assert_eq!(get_aoc_input(2025, 2025), "This is a test file :D");
    }
}
