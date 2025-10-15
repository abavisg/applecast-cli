
# Step 2: Fetch Episode Page HTML – `applecast-cli`

## 🎯 Goal
Once a valid Apple Podcasts URL is passed, download the full HTML content of that page using Rust.

## 📦 Features

- Accept the validated Apple Podcasts URL
- Perform an HTTP GET request
- Save the full HTML response to a local file (`episode.html`)
- Handle and log HTTP errors

## 🧱 Technical Stack

- Language: Rust
- HTTP client: [`reqwest`](https://crates.io/crates/reqwest) (blocking mode)
- File I/O: `std::fs`

## 🧪 Example Usage

```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Expected:
- Console shows: "✅ Fetched HTML content."
- File `episode.html` is created in `output/`

## 📁 File Structure

```
applecast-cli/
  ├── src/
  │   └── main.rs
  ├── output/
  │   └── episode.html
  └── Cargo.toml
```

## ✅ Success Criteria

- `episode.html` exists after execution
- File contains readable HTML of the episode/show page
- Tool gracefully handles bad URLs or failed requests

## 📌 Next Step

Extract episode metadata (title, description, publish date, etc.) from the HTML (Step 3).
