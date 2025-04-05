use std::{fmt::LowerHex, path::Path};

use digest::Output;
use md5::Digest;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
  #[cfg(feature = "md5")]
  MD5,
  #[cfg(feature = "sha1")]
  SHA1,
  #[cfg(feature = "sha2")]
  SHA256,
  #[cfg(feature = "sha2")]
  SHA512,
  #[cfg(feature = "sha3")]
  SHA3_256,
  #[cfg(feature = "blake2")]
  Blake2b,
  #[cfg(feature = "blake2")]
  Blake2s,
  #[cfg(feature = "blake3")]
  Blake3,
}

#[derive(Debug, TypedBuilder)]
pub struct Hashery {
  algorithm: Algorithm,
  #[builder(default = 1024 * 1024)] // 默认 1MB
  buffer_size: usize,
}

impl Hashery {
  pub async fn digest<P: AsRef<Path>>(&self, path: P) -> Result<String, std::io::Error> {
    let hash = match self.algorithm {
      #[cfg(feature = "md5")]
      Algorithm::MD5 => self.make_digest::<md5::Md5, P>(path).await?,
      #[cfg(feature = "sha1")]
      Algorithm::SHA1 => self.make_digest::<sha1::Sha1, P>(path).await?,
      #[cfg(feature = "sha2")]
      Algorithm::SHA256 => self.make_digest::<sha2::Sha256, P>(path).await?,
      #[cfg(feature = "sha2")]
      Algorithm::SHA512 => self.make_digest::<sha2::Sha512, P>(path).await?,
      #[cfg(feature = "sha3")]
      Algorithm::SHA3_256 => self.make_digest::<sha3::Sha3_256, P>(path).await?,
      #[cfg(feature = "blake2")]
      Algorithm::Blake2b => self.make_digest::<blake2::Blake2b512, P>(path).await?,
      #[cfg(feature = "blake2")]
      Algorithm::Blake2s => self.make_digest::<blake2::Blake2s256, P>(path).await?,
      #[cfg(feature = "blake3")]
      Algorithm::Blake3 => self.make_blake3(path).await?,
    };

    Ok(hash)
  }

  async fn make_digest<D: Digest, P: AsRef<Path>>(&self, path: P) -> Result<String, std::io::Error>
  where
    Output<D>: LowerHex,
  {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = D::new();
    let mut buffer = vec![0; 1024 * 1024];

    loop {
      let n = file.read(&mut buffer).await?;
      if n == 0 {
        break;
      }
      hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
  }

  #[cfg(feature = "blake3")]
  async fn make_blake3<P: AsRef<Path>>(&self, path: P) -> Result<String, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let file = tokio::fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);

    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0; self.buffer_size];

    while let Ok(n) = reader.read(&mut buffer).await {
      if n == 0 {
        break;
      }
      hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize().to_hex().to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // 测试默认开启的 MD5 feature
  #[cfg(feature = "md5")]
  #[tokio::test]
  async fn test_md5() {
    let hashery = Hashery::builder().algorithm(Algorithm::MD5).build();
    let hash = hashery.digest("fixtures/demo1.txt").await.unwrap();
    println!("{:?}", hash);
    assert!(!hash.is_empty());
  }

  // 测试 SHA3 feature（默认未开启）
  #[cfg(feature = "sha3")]
  #[tokio::test]
  async fn test_sha3() {
    let hashery = Hashery::builder().algorithm(Algorithm::SHA3_256).build();
    let hash = hashery.digest("fixtures/demo1.txt").await.unwrap();
    println!("{:?}", hash);
    assert!(!hash.is_empty());
  }

  // 测试 Blake2 feature（默认未开启）
  #[cfg(feature = "blake2")]
  #[tokio::test]
  async fn test_blake2() {
    // 测试 Blake2b
    let hashery = Hashery::builder().algorithm(Algorithm::Blake2b).build();
    let hash = hashery.digest("fixtures/demo1.txt").await.unwrap();
    assert!(!hash.is_empty());

    // 测试 Blake2s
    let hashery = Hashery::builder().algorithm(Algorithm::Blake2s).build();
    let hash = hashery.digest("fixtures/demo1.txt").await.unwrap();
    assert!(!hash.is_empty());
  }
}
