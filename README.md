# applecast-cli

A command-line tool for fetching and processing Apple Podcasts content.

## Installation

Clone the repository and build:

```bash
git clone <repository-url>
cd applecast-cli
cargo build --release
```

## Usage

### Basic Usage

Provide an Apple Podcasts URL to fetch and save the episode page HTML:

```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Output:
```
📥 Received URL: https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
✅ Fetched HTML content.
```

The HTML content will be saved to `output/episode.html`.

### Examples

Fetch episode page:
```bash
applecast-cli https://podcasts.apple.com/us/podcast/the-daily/id1200361736?i=1000631244436
```

Fetch show page:
```bash
applecast-cli https://podcasts.apple.com/us/podcast/the-daily/id1200361736
```

### Output

The tool creates an `output/` directory and saves the HTML content as `episode.html`:

```
applecast-cli/
├── output/
│   └── episode.html    # Full HTML content from Apple Podcasts
└── ...
```

### Error Handling

The tool provides clear error messages for common issues:

```bash
# Invalid URL format
applecast-cli not-a-valid-url
# Error: Invalid URL format: 'not-a-valid-url'

# HTTP errors (404, 500, etc.)
applecast-cli https://podcasts.apple.com/invalid-url
# Error: HTTP request failed with status: 404 Not Found

# Network errors
applecast-cli https://invalid-domain-12345.com
# Error: Failed to fetch URL: ...
```

## Development

Run tests:
```bash
cargo test
```

Format code:
```bash
cargo fmt
```

Lint code:
```bash
cargo clippy
```

## License

MIT