# applecast-cli

A command-line tool for fetching and processing podcast episode metadata from Apple Podcasts and other podcast websites.

## Installation

Clone the repository and build:

```bash
git clone <repository-url>
cd applecast-cli
cargo build --release
```

## Usage

### Basic Usage

Provide any podcast episode URL to fetch the page and extract metadata:

**Apple Podcasts:**
```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

**Other podcast websites:**
```bash
applecast-cli https://www.produxlabs.com/product-thinking-blog/episode-252-project-product-management
```

Output:
```
📥 Received URL: https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
✅ Fetched HTML content.
✅ Metadata extracted and saved to output/metadata.json
⚠️ No transcript found for this episode.
```

Or if a transcript is available:
```
📥 Received URL: https://podcasts.apple.com/episode-with-transcript
✅ Fetched HTML content.
✅ Metadata extracted and saved to output/metadata.json
✅ Transcript downloaded and saved to output/transcript.ttml
```

The tool automatically:
1. Fetches the HTML content and saves it to `output/episode.html`
2. Extracts episode metadata and saves it to `output/metadata.json`
3. Detects and downloads transcripts (`.ttml` format) if available

### Examples

**Apple Podcasts episode:**
```bash
applecast-cli https://podcasts.apple.com/us/podcast/the-daily/id1200361736?i=1000631244436
```

**Squarespace podcast blog:**
```bash
applecast-cli https://www.produxlabs.com/product-thinking-blog/episode-252-project-product-management
```

**Any podcast website with meta tags:**
The tool works with any website that has standard HTML meta tags (og:title, og:description, itemprop, etc.)

### Output

The tool creates an `output/` directory with the following files:

```
applecast-cli/
├── output/
│   ├── episode.html       # Full HTML content from Apple Podcasts
│   ├── metadata.json      # Extracted episode metadata
│   └── transcript.ttml    # Episode transcript (if available)
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

**transcript.ttml** (when available) contains the episode's closed captions in TTML format, which can be further processed or converted to plain text.

### Transcript Availability

Not all Apple Podcasts episodes include transcripts. The tool will automatically:
- Search for transcript URLs in the episode's HTML data
- Download the transcript if found (saved as `output/transcript.ttml`)
- Display a warning message if no transcript is available

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