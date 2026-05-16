pub fn parse_txt(path: String) -> Vec<String> {
    std::fs::read_to_string(path)
        .expect("Failed to open txt file")
        .split_whitespace()
        .map(String::from)
        .map(|s| s + ":443")
        .collect::<Vec<String>>()
}
