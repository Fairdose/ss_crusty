mod cli;
mod scrape;
mod io;

use cli::parse_args_and_client;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct PageResult {
    url: String,
    links: Vec<String>,
    html: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (args, client) = parse_args_and_client()?;

    let mut urls = args.urls.clone();
    if let Some(file_path) = &args.file {
        let content = io::read_file(file_path)?;
        urls.extend(parsers::parser_json(&content)); // extend with your chosen parser
    }
    urls.sort();
    urls.dedup();

    let results: Vec<PageResult> = urls
        .par_iter()
        .map(|url| {
            let (html, links) = scrape::fetch_and_extract(&client, url);
            PageResult {
                url: url.to_string(),
                links,
                html,
            }
        })
        .collect();

    let wrapped = serde_json::json!({ "pages": results });
    io::write_file(&args.output, &serde_json::to_string_pretty(&wrapped)?)?;
    println!("Results written to {}", args.output);

    Ok(())
}
