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

## Examples

Using different algorithms:

```rust
// SHA256 (requires "sha2" feature)
let hashery = Hashery::builder()
    .algorithm(Algorithm::SHA256)
    .build();

// BLAKE3 (requires "blake3" feature)
let hashery = Hashery::builder()
    .algorithm(Algorithm::Blake3)
    .build();
```

## Performance

The library uses asynchronous I/O and configurable buffer sizes for optimal performance. Default buffer size is 1MB, which can be adjusted based on your needs.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 