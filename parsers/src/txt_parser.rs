/// Parses a TXT string where URLs are separated by `;`
///
/// # Example
///
/// ```
/// let txt = "https://example.com;https://rust-lang.org;";
/// let urls = parsers::txt_parser::parse_txt(txt);
/// assert_eq!(urls, vec!["https://example.com".to_string(), "https://rust-lang.org".to_string()]);
/// ```
pub fn parse_txt(content: &str) -> Vec<String> {
    content
        .split(';')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}
