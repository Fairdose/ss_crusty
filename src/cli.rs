//! **cli.rs** handles all user interaction via the command line and configures the HTTP environment ⚙️.
//!
//! ### Responsibilities:
//!
//! * **Argument Parsing:** Defines the command-line interface using the `clap` crate, including options for input (`--urls`, `--file`), output (`--output`), and logging control (`--verbose`).
//! * **User Agent Management:** Provides a choice of common User Agents (`Mozilla`, `Webkit`, `Chrome`) to allow users to customize requests and potentially avoid being blocked by target websites.
//! * **Client Initialization:** Builds and returns a thread-safe, blocking `reqwest` HTTP client (`Arc<Client>`), pre-configured with the chosen User Agent, ready for concurrent use by the scraping routines.

use clap::{Parser, ValueEnum};
use reqwest::blocking::Client;
use std::sync::Arc;

/// Supported User Agents for HTTP requests, allowing the user to disguise or specify the client making the request.
#[derive(ValueEnum, Clone, Debug)]
pub enum UserAgent {
    Mozilla,
    Webkit,
    Chrome,
}

/// Command-line arguments for the ss_crusty application.
///
/// This structure defines all the configurable inputs the user can provide to control the scraping process.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Fetch URLs and extract links\nVerbose usage: -v = Info, -vv = Debug, -vvv = Trace"
)]
pub struct Args {
    /// URLs to fetch (can pass multiple `--urls "https://example1.com" --urls "https://example2.com"`)
    #[arg(long, action = clap::ArgAction::Append)]
    pub urls: Vec<String>,

    /// Path to one or more files containing URLs (can pass multiple `--file "file1.txt" --file "file2.json"`)
    #[arg(long, action = clap::ArgAction::Append)]
    pub file: Vec<String>,

    /// Output JSON file
    #[arg(long, default_value = "results.json")]
    pub output: String,

    /// Optional user-agent
    #[arg(long, value_enum)]
    pub user_agent: Option<UserAgent>,

    /// Enable verbose logging (-v = Info, -vv = Debug, -vvv = Trace)
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

/// Converts a `UserAgent` enum into an HTTP User-Agent string
///
/// # Examples
///
/// ```
/// use cli::{UserAgent, get_user_agent_string};
/// assert_eq!(get_user_agent_string(Some(&UserAgent::Mozilla)),
///            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Firefox/120.0");
/// assert_eq!(get_user_agent_string(None),
///            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
/// AppleWebKit/537.36 (KHTML, like Gecko) \
/// Chrome/120.0.0.0 Safari/537.36");
/// ```
pub fn get_user_agent_string(ua: Option<&UserAgent>) -> &str {
    match ua {
        Some(UserAgent::Mozilla) => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Firefox/120.0",
        Some(UserAgent::Webkit) => "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.4 Safari/605.1.15",
        Some(UserAgent::Chrome) => "Chrome/120.0.0.0 Safari/537.36",
        None => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/120.0.0.0 Safari/537.36",
    }
}

/// Parses command-line arguments and builds an HTTP client with the chosen User-Agent
///
/// # Examples
///
/// ```no_run
/// use cli::parse_args_and_client;
/// let (args, client) = parse_args_and_client().unwrap();
/// assert!(args.output.ends_with(".json"));
/// ```
pub fn parse_args_and_client() -> Result<(Args, Arc<Client>), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Arc::new(
        Client::builder()
            .user_agent(get_user_agent_string(args.user_agent.as_ref()))
            .build()?,
    );
    Ok((args, client))
}
