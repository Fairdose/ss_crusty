pub mod parser_json;
pub mod parser_xml;
pub mod parser_csv;
pub mod parser_txt;

use std::path::Path;

pub fn parse_file_content(file_path: &str, content: &str) -> Vec<String> {
    match Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "json" => parser_json::parser_json(content),
        "xml" => parser_xml::parser_xml(content),
        "csv" => parser_csv::parser_csv(content),
        "txt" => parser_txt::parser_txt(content),
        ext => {
            eprintln!("⚠️ Unsupported file type: {} (from {})", ext, file_path);
            Vec::new()
        }
    }
}
