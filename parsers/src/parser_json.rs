/// Parses a JSON string and extracts URLs from the `urls` array
///
/// # Example
///
/// ```
/// let json = r#"{ "urls": ["https://example.com", "https://rust-lang.org"] }"#;
/// let urls = parsers::parser_json(json);
/// assert_eq!(urls, vec!["https://example.com".to_string(), "https://rust-lang.org".to_string()]);
/// ```

use serde_json::Value;

pub fn parser_json(content: &str) -> Vec<String> {
    let v: Value = serde_json::from_str(content).unwrap_or_default();
    v.get("urls")
        .and_then(|urls| urls.as_array())
        .map(|arr| arr.iter().filter_map(|u| u.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}
