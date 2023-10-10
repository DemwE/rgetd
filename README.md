# rgetd
[![GitHub tag](https://img.shields.io/github/tag/DemwE/rget?include_prereleases=&sort=semver&color=blue)](https://github.com/DemwE/rget/releases/)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue)](#license)

rgetd is a simple command-line program written in Rust that allows you to download files from a given URL. It utilizes the reqwest crate for making HTTP requests and indicatif crate for displaying a progress bar during the download process.

### To Do

- [x] Progress bar
- [ ] Downloading multiple files at once
- [x] Own file name after download

### How to build?

```bash
git clone https://github.com/DemwE/rget
cd rget
cargo build --release
cp /target/release/rget .

# if you want to install on your system
cp rget /bin
```

[default config.toml](https://gist.github.com/DemwE/6de199a9d2febee7bd88f6a148d7a57c)
