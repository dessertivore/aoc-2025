pub fn split_lines(text_to_split: String) -> Vec<String> {
    let iterated: Vec<String> = text_to_split.lines().map(|line| line.to_string()).collect();
    return iterated;
}

pub fn split_string_by_specified_char(text_to_split: String, to_split_by: &str) -> Vec<String> {
    let iterator = text_to_split
        .split(to_split_by)
        .map(|x| x.to_string())
        .collect();
    return iterator;
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
