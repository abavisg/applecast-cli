use clap::Parser;
use std::fs;
use std::path::Path;
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

    // Fetch HTML content
    let output_path = "output/episode.html";
    if let Err(e) = fetch_html(&args.url, output_path) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("✅ Fetched HTML content.");
}

/// Validates that the provided string is a valid URL
fn validate_url(url_str: &str) -> Result<(), String> {
    Url::parse(url_str)
        .map(|_| ())
        .map_err(|_| format!("Invalid URL format: '{}'", url_str))
}

/// Fetches HTML content from a URL and saves it to a file
fn fetch_html(url: &str, output_path: &str) -> Result<(), String> {
    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Fetch HTML content
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    // Check if response is successful
    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status: {}", response.status()));
    }

    // Get response body
    let html = response
        .text()
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // Save to file
    fs::write(output_path, html)
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
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

    /// Unit test - Fetch HTML creates output file with content
    #[test]
    fn test_fetch_html_creates_file_with_content() {
        use std::fs;
        use tempfile::TempDir;

        // Given a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_output.html");
        let output_path_str = output_path.to_str().unwrap();

        // When we fetch from a test URL (httpbin.org returns HTML)
        let result = fetch_html("https://httpbin.org/html", output_path_str);

        // Then the fetch succeeds
        assert!(result.is_ok(), "fetch_html should succeed");

        // And the file exists
        assert!(output_path.exists(), "Output file should exist");

        // And the file contains HTML content
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("html"), "File should contain HTML content");
        assert!(!content.is_empty(), "File should not be empty");
    }

    /// Unit test - Fetch HTML creates output directory if missing
    #[test]
    fn test_fetch_html_creates_directory() {
        use tempfile::TempDir;

        // Given a temporary directory with a nested path that doesn't exist
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("nested").join("output.html");
        let output_path_str = output_path.to_str().unwrap();

        // When we fetch HTML
        let result = fetch_html("https://httpbin.org/html", output_path_str);

        // Then the fetch succeeds
        assert!(result.is_ok(), "fetch_html should create nested directories");

        // And the nested directory was created
        assert!(output_path.parent().unwrap().exists(), "Parent directory should be created");

        // And the file exists
        assert!(output_path.exists(), "Output file should exist in nested directory");
    }

    /// Unit test - Fetch HTML handles invalid URL gracefully
    #[test]
    fn test_fetch_html_handles_request_error() {
        use tempfile::TempDir;

        // Given a temporary directory and an invalid URL
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.html");
        let output_path_str = output_path.to_str().unwrap();

        // When we try to fetch from an invalid domain
        let result = fetch_html("https://thisisnotavaliddomainforsurehopefully123456789.com", output_path_str);

        // Then the fetch fails
        assert!(result.is_err(), "fetch_html should fail for invalid domains");

        // And the error message is descriptive
        let error = result.unwrap_err();
        assert!(error.contains("Failed to fetch URL"), "Error should mention fetch failure");
    }

    /// Unit test - Fetch HTML handles HTTP error status codes
    #[test]
    fn test_fetch_html_handles_http_error_status() {
        use tempfile::TempDir;

        // Given a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.html");
        let output_path_str = output_path.to_str().unwrap();

        // When we try to fetch a URL that returns 404
        let result = fetch_html("https://httpbin.org/status/404", output_path_str);

        // Then the fetch fails
        assert!(result.is_err(), "fetch_html should fail for HTTP error codes");

        // And the error message mentions the status
        let error = result.unwrap_err();
        assert!(error.contains("404"), "Error should mention the HTTP status code");
    }
}
