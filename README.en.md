# i_edit_json

[![Crates.io](https://img.shields.io/crates/v/i_edit_json)](https://crates.io/crates/i_edit_json)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg)](https://www.rust-lang.org)

A lightweight, high-performance JSON field extraction and manipulation tool.

## Features

- Extract and modify JSON fields using intuitive dot-separated paths
- Support for nested structures and array operations
- Type-aware value handling
- Can be used as both a CLI tool and a Rust library
- Convenience functions for common JSON operations

## Installation

### From crates.io

```bash
cargo install i_edit_json
```

### From source

```bash
git clone https://github.com/your-username/i_edit_json
cd i_edit_json
cargo install --path .
```

```bash
git clone https://github.com/ymc-github/i_edit_json
cd i_edit_json
cargo install --path .
```

```bash
cargo install --git https://github.com/ymc-github/i_edit_json

cargo install --git https://github.com/ymc-github/i_edit_json --branch main

cargo install --git https://github.com/ymc-github/i_edit_json --tag v0.1.0
```

### From docker hub
```bash
#  from docker.io
# docker pull docker.io/ymc-github/i_edit_json:latest
docker pull ymc-github/i_edit_json:latest

#  from ghcr.io
# ghcr.io/<owner>/<repo>:latest
docker pull ghcr.io/ymc-github/i_edit_json:latest
```


## Usage

### As a CLI Tool

#### Extract Fields (get command)

```bash
# Extract package name from package.json
i_edit_json get --field name

# Strip quotes from string values
i_edit_json get --field name --strip-quotes
i_edit_json get --field version --strip-quotes
i_edit_json get --field description --strip-quotes
i_edit_json get --field repository.url --strip-quotes
i_edit_json get --field "keywords" --strip-quotes
i_edit_json get --field "author" --strip-quotes


```

#### Set Fields (set command)

```bash
i_edit_json set -f package.json -k version -v "1.1.0" --in-place
# update description
i_edit_json set -f package.json -k description -v "An enhanced Node.js application" --in-place

# update author
# i_edit_json set -f package.json -k author -v "Ye Mincong <cong@example.com>" --in-place

# update license
# i_edit_json set -f package.json -k license -v "Apache-2.0" --in-place

# Set array element
i_edit_json set -k keywords[0] -v "rust" --in-place

# Specify value type
i_edit_json set -k private -v "true" -t boolean --in-place
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
i_edit_json = "0.1"
```

Use in your code:

```rust
use i_edit_json::{get, set, ExtractConfig, SetConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Extract field
    let get_config = ExtractConfig {
        file_path: "package.json".to_string(),
        field_path: "version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    let version = get::extract_field(&get_config)?;
    println!("Current version: {}", version);

    // Set field
    let set_config = SetConfig {
        file_path: "package.json".to_string(),
        field_path: "version".to_string(),
        value: "1.2.3".to_string(),
        value_type: None,
        create_missing: false,
    };
    set::set_field_and_save(&set_config)?;
    println!("Version updated successfully");

    Ok(())
}
```

## License

MIT OR Apache-2.0
