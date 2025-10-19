use clap::{Parser, ValueEnum};
use reqwest::blocking::Client;
use std::sync::Arc;

/// Supported User Agents for HTTP requests
#[derive(ValueEnum, Clone, Debug)]
pub enum UserAgent {
    Mozilla,
    Webkit,
    Chrome,
}

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// URLs to fetch (can pass multiple `--urls`)
    #[arg(long, action = clap::ArgAction::Append)]
    pub urls: Vec<String>,

    /// Path to a file containing URLs
    #[arg(long)]
    pub file: Option<String>,

    /// Output JSON file
    #[arg(long, default_value = "results.json")]
    pub output: String,

    /// Optional user-agent
    #[arg(long, value_enum)]
    pub user_agent: Option<UserAgent>,
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
