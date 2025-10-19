#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{parser_json, parser_txt, parser_csv, parser_xml};
    const TEST_DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data");

    fn expected_urls() -> Vec<String> {
        vec![
            "https://example.com".to_string(),
            "https://en.wikipedia.org/wiki/Online_magazine".to_string(),
        ]
    }

    #[test]
    fn test_json_parser() {
        let content = fs::read_to_string(format!("{}/urls.json", TEST_DATA_PATH))
            .expect("Failed to read JSON test file");
        assert_eq!(parser_json(&content), expected_urls());
    }

    #[test]
    fn test_txt_parser() {
        let content = fs::read_to_string(format!("{}/urls.txt", TEST_DATA_PATH))
            .expect("Failed to read TXT test file");
        assert_eq!(parser_txt(&content), expected_urls());
    }

    #[test]
    fn test_csv_parser() {
        let content = fs::read_to_string(format!("{}/urls.csv", TEST_DATA_PATH))
            .expect("Failed to read CSV test file");
        assert_eq!(parser_csv(&content), expected_urls());
    }

    #[test]
    fn test_xml_parser() {
        let content = fs::read_to_string(format!("{}/urls.xml", TEST_DATA_PATH))
            .expect("Failed to read XML test file");
        assert_eq!(parser_xml(&content), expected_urls());
    }
}
