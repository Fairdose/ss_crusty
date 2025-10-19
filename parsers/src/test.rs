#[cfg(test)]
mod tests {
    use super::*;
    use crate::{json_parser, txt_parser, xml_parser, csv_parser};

    #[test]
    fn test_json_parser() {
        let content = r#"{ "urls": ["http://example.com", "http://example2.com"] }"#;
        assert_eq!(json_parser::parse_json(content), vec!["http://example.com", "http://example2.com"]);
    }

    #[test]
    fn test_txt_parser() {
        let content = "http://example.com;http://example2.com;";
        assert_eq!(txt_parser::parse_txt(content), vec!["http://example.com", "http://example2.com"]);
    }

    #[test]
    fn test_xml_parser() {
        let content = r#"<urls><url>http://example.com</url><url>http://example2.com</url></urls>"#;
        assert_eq!(xml_parser::parse_xml(content), vec!["http://example.com", "http://example2.com"]);
    }

    #[test]
    fn test_csv_parser() {
        let content = "url\nhttp://example.com\nhttp://example2.com\n";
        assert_eq!(csv_parser::parse_csv(content), vec!["http://example.com", "http://example2.com"]);
    }
}