//! ss_crusty (Simple Scraper) is a high-performance, command-line application written in **Rust** 🦀 designed to **fetch HTML content from a list of URLs in parallel and extract all absolute links**.
//! [GitHub Repo](https://github.com/Fairdose/ss_crusty)
//!
//! ### Core Functionality
//!
//! 1.  **Input Versatility:** Accepts target URLs both directly via command-line arguments and from external files.
//! 2.  **Concurrency (Multi-threading):** Utilizes the **[`rayon`](https://docs.rs/rayon/latest/rayon/)** library for **parallel execution** (`par_iter().map(...)`), enabling the application to fetch and scrape multiple URLs concurrently across available CPU cores, ensuring fast and efficient processing of large lists.
//! 3.  **Data Extraction:** Employs the [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) HTTP client and the [`scraper`](https://docs.rs/scraper/latest/scraper/) parsing library to robustly fetch and analyze web pages.
//! 4.  **Structured Output:** Gathers all results (original URL, raw HTML, and unique extracted links) and serializes them into a single, clean **JSON file** using [`serde`](https://docs.rs/serde/latest/serde/).
//!
//! ---
//!
//! #### Arguments
//!
//! * **`--urls <URL>`**: URLs to fetch. **Must be repeated** for each URL to be added (e.g., `--urls "url1" --urls "url2"`).
//! * **`--file <PATH>`**: Path to one or more files containing URLs. **Must be repeated** for each file (e.g., `--file "list1.txt" --file "list2.txt"`).
//! * **`--output <PATH>`**: The output JSON file name (defaults to `results.json`).
//! * **`--user-agent <AGENT:String>`**: Optional user-agent string override (`Mozilla`, `Webkit`, or `Chrome`).
//! * **`-v, -vv, -vvv`**: Controls logging level (`-v` = Info, `-vv` = Debug, `-vvv` = Warn).
//!
//! ---
//!
//! #### Usage
//!
//! Run the application with URLs and specify the output file:
//!
//! ```bash
//! ss_crusty --urls "https://example.com" --file "list.txt" --output "results.json" -vv
//! ```
mod cli;
mod scrape;
mod io;

use cli::parse_args_and_client;
use rayon::prelude::*;
use serde::Serialize;
use log::{info, debug};

#[derive(Serialize)]
/// Represents the result of scraping a single URL.
/// This structure is used by the application to serialize the final data into the output JSON file.
struct PageResult {
    /// The URL that was fetched.
    url: String,
    /// A list of all unique, absolute links extracted from the page's HTML.
    links: Vec<String>,
    /// The raw HTML content of the page.
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
