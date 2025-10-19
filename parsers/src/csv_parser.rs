/// Parses CSV content where URLs are in a column named `url`
///
/// # Example
///
/// ```
/// let csv = "url\nhttps://example.com\nhttps://rust-lang.org\n";
/// let urls = parsers::csv_parser::parse_csv(csv);
/// assert_eq!(urls, vec!["https://example.com".to_string(), "https://rust-lang.org".to_string()]);
/// ```

use csv::ReaderBuilder;
use std::io::Cursor;

pub fn parse_csv(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(content));

    for result in rdr.records() {
        if let Ok(record) = result {
            if let Some(url) = record.get(0) {
                urls.push(url.to_string());
            }
        }
    }

    urls
}
