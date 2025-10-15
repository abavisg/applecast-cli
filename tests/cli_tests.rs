use assert_cmd::Command;
use predicates::prelude::*;

/// Scenario - Valid URL provided
/// Given a valid Apple Podcasts URL
/// When user runs `applecast-cli <url>`
/// Then output shows "📥 Received URL: <url>"
#[test]
fn test_valid_url_prints_received_message() {
    let mut cmd = Command::cargo_bin("applecast-cli").unwrap();
    let test_url = "https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436";

    cmd.arg(test_url)
        .assert()
        .success()
        .stdout(predicate::str::contains("📥 Received URL:"))
        .stdout(predicate::str::contains(test_url));
}

/// Scenario - No URL provided
/// Given no arguments
/// When user runs `applecast-cli`
/// Then error message and usage help displayed
#[test]
fn test_no_url_shows_error() {
    let mut cmd = Command::cargo_bin("applecast-cli").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

/// Scenario - Invalid URL format
/// Given an invalid URL string
/// When user runs `applecast-cli not-a-url`
/// Then error message about invalid URL
#[test]
fn test_invalid_url_shows_error() {
    let mut cmd = Command::cargo_bin("applecast-cli").unwrap();

    cmd.arg("not-a-valid-url")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid URL"));
}

/// Scenario - Valid Apple Podcasts show URL
/// Given a valid Apple Podcasts show URL (without episode ID)
/// When user runs `applecast-cli <url>`
/// Then output shows the URL was received
#[test]
fn test_valid_show_url_accepted() {
    let mut cmd = Command::cargo_bin("applecast-cli").unwrap();
    let show_url = "https://podcasts.apple.com/us/podcast/id840986946";

    cmd.arg(show_url)
        .assert()
        .success()
        .stdout(predicate::str::contains("📥 Received URL:"))
        .stdout(predicate::str::contains(show_url));
}
