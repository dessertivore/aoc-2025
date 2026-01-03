/// Splits the given text by line breaks.
///
/// # Parameters
/// - `text_to_split`: The input string to be split.
///
/// # Returns
/// An iterator of `String` values, where each element is a part of the input string
/// split by the specified delimiter.
///
/// # Example
/// ```rust
/// use aoc_2025::utils::parsing;
/// let text = "apple \n banana \n cherry".to_string();
/// let result: Vec<String> = parsing::split_lines(text);
/// println!("{:?}",result)
/// ```
pub fn split_lines(text_to_split: String) -> Vec<String> {
    text_to_split.lines().map(|line| line.to_string()).collect()
}

/// Splits the given text into an iterator of strings based on the specified delimiter.
///
/// # Parameters
/// - `text_to_split`: The input string to be split.
/// - `to_split_by`: The delimiter used to split the input string.
///
/// # Returns
/// An iterator of `String` values, where each element is a part of the input string
/// split by the specified delimiter.
///
/// # Example
/// ```rust
/// use aoc_2025::utils::parsing;
/// let text = "apple,banana,cherry".to_string();
/// let delimiter = ",";
/// let result: Vec<String> = parsing::split_string_by_specified_char(text, delimiter);
/// ```
pub fn split_string_by_specified_char(text_to_split: String, to_split_by: &str) -> Vec<String> {
    let iterator = text_to_split
        .split(to_split_by)
        .map(|x| x.to_string())
        .collect();

    iterator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines() {
        let split: Vec<String> = split_lines("This should be \n split in two!".to_string());
        assert_eq!(split, ["This should be ", " split in two!"]);
    }

    #[test]
    fn test_split_by_char() {
        let split: String = "This should be *insert random string here* split in two!".to_string();
        assert_eq!(
            split_string_by_specified_char(split, "*insert random string here*"),
            ["This should be ", " split in two!"]
        )
    }
}
