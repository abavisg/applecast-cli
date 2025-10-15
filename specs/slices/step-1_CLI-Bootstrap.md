
# Step 1: CLI Bootstrap – `applecast-cli`

## 🎯 Goal
Create the initial CLI structure for a Rust tool that accepts a single Apple Podcasts URL (either a show or an episode) as a command-line argument and prints it to the console.

## 📦 Features

- Parse one required argument: the podcast or episode URL
- Validate the argument (basic URL format)
- Print the URL if valid
- Print an error message if missing or malformed

## 🧱 Technical Stack

- Language: Rust
- CLI library: [`clap`](https://crates.io/crates/clap) (recommended version 4.x)

## 🧪 Example Usage

```bash
applecast-cli https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

Output:
```
📥 Received URL: https://podcasts.apple.com/us/podcast/id840986946?i=1000631244436
```

## 📁 File Structure

```
applecast-cli/
  ├── src/
  │   └── main.rs
  └── Cargo.toml
```

## ✅ Success Criteria

- Project compiles successfully with `cargo build`
- Running with a valid URL prints the URL
- Running with no URL prints usage help or error

## 📌 Next Step

After this, you'll fetch the HTML of the given URL (Step 2).
