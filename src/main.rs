use clap::Parser;
use std::process;
use url::Url;

/// A CLI tool for fetching and processing Apple Podcasts content
#[derive(Parser, Debug)]
#[command(name = "applecast-cli")]
#[command(about = "Fetch and process Apple Podcasts episodes and shows", long_about = None)]
struct Args {
    /// Apple Podcasts episode or show URL
    #[arg(value_name = "URL")]
    url: String,
}

fn main() {
    let args = Args::parse();

    // Validate URL format
    if let Err(e) = validate_url(&args.url) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // Print the received URL
    println!("📥 Received URL: {}", args.url);
}

/// Validates that the provided string is a valid URL
fn validate_url(url_str: &str) -> Result<(), String> {
    Url::parse(url_str)
        .map(|_| ())
        .map_err(|_| format!("Invalid URL format: '{}'", url_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit test - Valid URL passes validation
    #[test]
    fn test_validate_url_accepts_valid_url() {
        let url = "https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436";
        assert!(validate_url(url).is_ok());
    }

    /// Unit test - Invalid URL fails validation
    #[test]
    fn test_validate_url_rejects_invalid_url() {
        let url = "not-a-valid-url";
        assert!(validate_url(url).is_err());
    }

    /// Unit test - HTTP URLs are valid
    #[test]
    fn test_validate_url_accepts_http() {
        let url = "http://example.com";
        assert!(validate_url(url).is_ok());
    }

    /// Unit test - Empty string fails validation
    #[test]
    fn test_validate_url_rejects_empty_string() {
        let url = "";
        assert!(validate_url(url).is_err());
    }

    /// Unit test - Error message includes the invalid URL
    #[test]
    fn test_validate_url_error_includes_url() {
        let url = "not-valid";
        let result = validate_url(url);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not-valid"));
    }
}
