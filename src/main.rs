use std::fs;
use std::sync::{Arc, Mutex, mpsc};
use rayon::prelude::*;
use parsers::{json_parser, txt_parser, xml_parser, csv_parser};
use std::thread;

fn parse_file_content(path_or_url: &str, content: &str) -> Vec<String> {
    if path_or_url.ends_with(".json") {
        json_parser::parse_json(content)
    } else if path_or_url.ends_with(".txt") {
        txt_parser::parse_txt(content)
    } else if path_or_url.ends_with(".xml") {
        xml_parser::parse_xml(content)
    } else if path_or_url.ends_with(".csv") {
        csv_parser::parse_csv(content)
    } else {
        vec![]
    }
}

fn read_content(path_or_url: &str) -> String {
    if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
        reqwest::blocking::get(path_or_url)
            .map(|resp| resp.text().unwrap_or_default())
            .unwrap_or_default()
    } else {
        fs::read_to_string(path_or_url).unwrap_or_default()
    }
}

fn main() {
    let paths_or_urls = vec![
        "data.json",
        "data.txt",
        "data.xml",
        "data.csv",
    ];

    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel();

    for path_or_url in paths_or_urls {
        let tx = tx.clone();
        let results = Arc::clone(&results);
        let path_or_url = path_or_url.to_string();

        thread::spawn(move || {
            let content = read_content(&path_or_url);
            let urls = parse_file_content(&path_or_url, &content);

            {
                let mut res = results.lock().unwrap();
                res.extend(urls.clone());
            }

            tx.send(format!("Processed {}", path_or_url)).unwrap();
        });
    }

    drop(tx);

    for msg in rx {
        println!("{}", msg);
    }

    let final_urls = Arc::clone(&results);
    final_urls.lock().unwrap().par_iter().for_each(|url| {
        println!("Found URL: {}", url);
    });

    println!("All URLs processed. Total: {}", results.lock().unwrap().len());
}
