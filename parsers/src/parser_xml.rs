/// Parses XML content and extracts URLs from `<url>` tags
///
/// # Example
///
/// ```
/// use parsers::parser_xml;
///
/// let xml = r#"<root><url>https://example.com</url><url>https://rust-lang.org</url></root>"#;
/// let urls = parser_xml(xml);
/// assert_eq!(urls, vec!["https://example.com".to_string(), "https://rust-lang.org".to_string()]);
/// ```

use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

pub fn parser_xml(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut reader = Reader::from_str(content);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name() == QName(b"url") => {
                if let Ok(Event::Text(t)) = reader.read_event() {
                    urls.push(t.unescape().unwrap_or_default().to_string());
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    urls
}
