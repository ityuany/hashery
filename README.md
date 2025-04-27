# Hashery

[![Crates.io](https://img.shields.io/crates/v/hashery.svg)](https://crates.io/crates/hashery)
[![Documentation](https://docs.rs/hashery/badge.svg)](https://docs.rs/hashery)
[![License](https://img.shields.io/crates/l/hashery.svg)](LICENSE)

[中文文档](README-zh_CN.md)

A flexible and efficient async file hashing library for Rust, supporting multiple hash algorithms.

## Features

- Asynchronous file hashing with Tokio
- Multiple hash algorithm support (MD5, SHA1, SHA2, SHA3, BLAKE2, BLAKE3)
- Configurable buffer size for optimal performance
- Feature flags for selective algorithm inclusion
- Builder pattern for easy configuration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hashery = "0.1"
```

By default, only MD5 is enabled. To use other algorithms, enable the corresponding features:

```toml
[dependencies]
hashery = { version = "0.1", features = ["sha2", "blake3"] }
```

## Available Features

- `md5` (default) - Enable MD5 hashing
- `sha1` - Enable SHA1 hashing
- `sha2` - Enable SHA256 and SHA512
- `sha3` - Enable SHA3-256
- `blake2` - Enable BLAKE2b and BLAKE2s
- `blake3` - Enable BLAKE3

Feature groups:

- `full` - Enable all supported algorithms
- `modern` - Enable modern algorithms (SHA2 and BLAKE3)

## Usage

```rust
use hashery::{Hashery, Algorithm};

#[tokio::main]
async fn main() -> std::io::Result<String> {
    // Create a hasher with MD5 algorithm
    let hashery = Hashery::builder()
        .algorithm(Algorithm::MD5)
        .buffer_size(1024 * 1024) // Optional: Set custom buffer size (default: 1MB)
        .build();

    // Calculate file hash
    let hash = hashery.digest("path/to/file").await?;
    println!("File hash: {}", hash);

    Ok(())
}
```

## Hashing Bytes and Strings

You can also hash bytes or strings directly:

```rust
let hashery = Hashery::builder().algorithm(Algorithm::SHA256).build();

// Hash a byte slice
env let hash = hashery.digest_bytes(b"hello world").unwrap();
println!("SHA256: {}", hash);

// Hash a string
let hash = hashery.digest_str("hello world").unwrap();
println!("SHA256: {}", hash);
```

## Examples

Using different algorithms:

```

```
