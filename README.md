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

Provide an Apple Podcasts URL as an argument:

```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Output:
```
📥 Received URL: https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

### Examples

Episode URL:
```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Show URL:
```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946
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