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
