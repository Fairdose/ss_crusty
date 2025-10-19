use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use log::{info, warn, debug};

/// Fetches HTML from a URL using the provided HTTP client
///
/// Returns `"Failed to fetch"` if any network error occurs.
///
/// # Examples
///
/// ```no_run
/// use scrape::fetch_html;
/// use reqwest::blocking::Client;
/// let client = Client::new();
/// let html = fetch_html(&client, "https://example.com");
/// assert!(html.len() > 0);
/// ```
pub fn fetch_html(client: &Client, url: &str) -> String {
    info!("Fetching: {}", url);

    match client.get(url).send().and_then(|resp| resp.text()) {
        Ok(html) => {
            debug!("Successfully fetched {} ({} bytes)", url, html.len());
            html
        }
        Err(e) => {
            warn!("Failed to fetch {}: {}", url, e);
            "Failed to fetch".to_string()
        }
    }
}

/// Extracts all unique links starting with `http://` or `https://` from HTML
///
/// # Examples
///
/// ```
/// use scrape::extract_links;
/// let html = r#"<a href="https://example.com">link</a>"#;
/// let links = extract_links(html);
/// assert_eq!(links, vec!["https://example.com"]);
/// ```
pub fn extract_links(html: &str) -> Vec<String> {
    debug!("Extracting links from HTML...");
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links_set = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.starts_with("http://") || href.starts_with("https://") {
                links_set.insert(href.to_string());
            }
        }
    }

    let count = links_set.len();
    debug!("Extracted {} unique links", count);
    links_set.into_iter().collect()
}

/// Convenience function to fetch HTML and extract links in one step
///
/// # Examples
///
/// ```no_run
/// use scrape::fetch_and_extract;
/// use reqwest::blocking::Client;
/// let client = Client::new();
/// let (html, links) = fetch_and_extract(&client, "https://example.com");
/// assert!(html.len() > 0);
/// assert!(links.len() >= 0);
/// ```
pub fn fetch_and_extract(client: &Client, url: &str) -> (String, Vec<String>) {
    info!("Processing: {}", url);

    let html = fetch_html(client, url);
    let links = extract_links(&html);

    info!("{} -> {} links extracted", url, links.len());
    (html, links)
}
