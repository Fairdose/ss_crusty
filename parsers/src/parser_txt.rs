/// Parses a TXT string and extracts URLs separated by newlines
///
/// # Example
///
/// ```
/// let txt = "https://example.com\r\nhttps://rust-lang.org";
/// let urls = parsers::parser_txt(txt);
/// assert_eq!(urls, vec!["https://example.com".to_string(), "https://rust-lang.org".to_string()]);
/// ```
pub fn parser_txt(content: &str) -> Vec<String> {
    content
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}
