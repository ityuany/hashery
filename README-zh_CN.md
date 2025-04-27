# Hashery

[![Crates.io](https://img.shields.io/crates/v/hashery.svg)](https://crates.io/crates/hashery)
[![Documentation](https://docs.rs/hashery/badge.svg)](https://docs.rs/hashery)
[![License](https://img.shields.io/crates/l/hashery.svg)](LICENSE)

[English](README.md)

一个灵活高效的 Rust 异步文件哈希库，支持多种哈希算法。

## 特性

- 基于 Tokio 的异步文件哈希计算
- 支持多种哈希算法（MD5、SHA1、SHA2、SHA3、BLAKE2、BLAKE3）
- 可配置的缓冲区大小，实现最佳性能
- 通过特性标志选择性启用算法
- 使用构建器模式实现简单配置

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
hashery = "0.1"
```

默认情况下只启用 MD5。要使用其他算法，需要启用相应的特性：

```toml
[dependencies]
hashery = { version = "0.1", features = ["sha2", "blake3"] }
```

## 可用特性

- `md5`（默认） - 启用 MD5 哈希
- `sha1` - 启用 SHA1 哈希
- `sha2` - 启用 SHA256 和 SHA512
- `sha3` - 启用 SHA3-256
- `blake2` - 启用 BLAKE2b 和 BLAKE2s
- `blake3` - 启用 BLAKE3

特性组合：

- `full` - 启用所有支持的算法
- `modern` - 启用现代算法（SHA2 和 BLAKE3）

## 使用方法

```rust
use hashery::{Hashery, Algorithm};

#[tokio::main]
async fn main() -> std::io::Result<String> {
    // 创建一个使用 MD5 算法的哈希器
    let hashery = Hashery::builder()
        .algorithm(Algorithm::MD5)
        .buffer_size(1024 * 1024) // 可选：设置自定义缓冲区大小（默认：1MB）
        .build();

    // 计算文件哈希
    let hash = hashery.digest("path/to/file").await?;
    println!("文件哈希值: {}", hash);

    Ok(())
}
```

## 示例

使用不同的算法：

```rust
// SHA256（需要 "sha2" 特性）
let hashery = Hashery::builder()
    .algorithm(Algorithm::SHA256)
    .build();

// BLAKE3（需要 "blake3" 特性）
let hashery = Hashery::builder()
    .algorithm(Algorithm::Blake3)
    .build();
```

## 性能

本库使用异步 I/O 和可配置的缓冲区大小来实现最佳性能。默认缓冲区大小为 1MB，可以根据需要调整。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 字节和字符串哈希

你也可以直接对字节切片或字符串进行哈希：

```rust
let hashery = Hashery::builder().algorithm(Algorithm::SHA256).build();

// 哈希字节切片
let hash = hashery.digest_bytes(b"hello world").unwrap();
println!("SHA256: {}", hash);

// 哈希字符串
let hash = hashery.digest_str("hello world").unwrap();
println!("SHA256: {}", hash);
```
