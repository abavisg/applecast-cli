
# Step 4: Detect and Download Transcript – `applecast-cli`

## 🎯 Goal
Detect and download the `.ttml` transcript file associated with an Apple Podcasts episode (if available) by inspecting the saved HTML (`output/episode.html`).

## 📦 Features

- Open `output/episode.html`
- Search for the `.ttml` URL in:
  - Inline JSON blobs inside `<script>` tags
  - Hardcoded links containing `.ttml`
- If a `.ttml` URL is found:
  - Download the file using `reqwest`
  - Save it as `output/transcript.ttml`
- Handle cases where no transcript is available

## 🧱 Technical Stack

- HTML + regex/string parsing: `scraper`, `regex`, or `html5ever`
- HTTP client: `reqwest`
- Error handling: `anyhow` recommended

## 🧪 Example Usage

```bash
applecast-cli https://podcasts.apple.com/episode-url
```

Expected:
- `output/transcript.ttml` is saved
- If not found, print: "⚠️ No transcript found for this episode."

## 📁 File Structure

```
applecast-cli/
  └── output/
      ├── episode.html
      └── transcript.ttml
```

## ✅ Success Criteria

- `.ttml` file is saved if URL is detected
- If not found, appropriate message is printed
- HTTP errors are handled cleanly

## 📌 Next Step

Parse `transcript.ttml` into a clean `transcript.txt` file (Step 5).
