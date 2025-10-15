
# Step 3: Extract Episode Metadata – `applecast-cli`

## 🎯 Goal
Parse key metadata from the previously downloaded Apple Podcasts HTML page (`episode.html`) and save it as structured JSON.

## 📦 Features

- Read `output/episode.html`
- Extract:
  - Episode title
  - Episode description
  - Show title (if available)
  - Publish date (if present)
  - Duration (optional)
- Save as `metadata.json` in the same `output/` folder

## 🧱 Technical Stack

- HTML parser: [`scraper`](https://crates.io/crates/scraper) crate (or `select`)
- Data format: [`serde_json`](https://crates.io/crates/serde_json)
- Structs: Use `serde::Serialize` to define a `Metadata` struct

## 🧪 Example Output (`metadata.json`)

```json
{
  "title": "Why Octopuses Hate the Moon",
  "description": "In this episode, we explore the behaviour of octopuses during lunar cycles...",
  "show": "No Such Thing As A Fish",
  "publish_date": "2023-11-25"
}
```

## 📁 File Structure

```
applecast-cli/
  ├── output/
  │   ├── episode.html
  │   └── metadata.json
```

## ✅ Success Criteria

- `metadata.json` is created with readable and accurate metadata
- Graceful handling if fields are missing (e.g. description or publish date)
- Fields are trimmed and clean (no HTML tags or excessive whitespace)

## 📌 Next Step

Look for `.ttml` transcript URLs in the HTML and download them (Step 4).
