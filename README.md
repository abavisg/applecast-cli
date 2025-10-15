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

Provide an Apple Podcasts URL to fetch the episode page and extract metadata:

```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Output:
```
📥 Received URL: https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
✅ Fetched HTML content.
✅ Metadata extracted and saved to output/metadata.json
```

The tool automatically:
1. Fetches the HTML content and saves it to `output/episode.html`
2. Extracts episode metadata and saves it to `output/metadata.json`

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

The tool creates an `output/` directory with two files:

```
applecast-cli/
├── output/
│   ├── episode.html      # Full HTML content from Apple Podcasts
│   └── metadata.json     # Extracted episode metadata
└── ...
```

**metadata.json** contains structured episode information:
```json
{
  "episode_title": "Kaepernick, Dak, the latest NBA news, and a slice of MLB",
  "description": "Join us as we discuss a few of the latest news...",
  "show_title": "Back to the Board",
  "publish_date": "2023-10-13"
}
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