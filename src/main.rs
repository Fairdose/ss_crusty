mod cli;
mod scrape;
mod io;

use cli::parse_args_and_client;
use rayon::prelude::*;
use serde::Serialize;
use log::{info, debug};

#[derive(Serialize)]
struct PageResult {
    url: String,
    links: Vec<String>,
    html: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (args, client) = parse_args_and_client()?;

    let log_level = match args.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    
    unsafe { std::env::set_var("RUST_LOG", format!("ss_crusty={}", log_level)); }
    env_logger::init();

    info!("Starting ss_crusty...");
    debug!("Arguments: {:?}", args);

    if args.urls.is_empty() && args.file.is_empty() {
        println!("No URLs or files provided.");
        println!("Verbose usage: -v (info), -vv (debug), -vvv (trace)");
        std::process::exit(1);
    }

    let mut urls = args.urls.clone();

    // Parse all input files
    for file_path in &args.file {
        info!("Reading file: {}", file_path);
        let content = io::read_file(file_path)?;
        let parsed = parsers::parse_file_content(file_path, &content);
        urls.extend(parsed);
    }

    urls.sort();
    urls.dedup();

    info!("Fetching {} unique URLs...", urls.len());

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
    info!("Results written to {}", args.output);
    info!("Processed {} input files", args.file.len());

    Ok(())
}
