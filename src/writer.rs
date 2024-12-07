//! This module is optional and can be enabled using the `writer` Cargo feature.
//!
//! The [`Writer`] allows on-the-fly calculation of the digest while writing the data.
//!
//! # Enabling
//!
//! Add the following entry to your `Cargo.toml` file to enable the `writer` feature:
//!
//! ```toml
//! [dependencies]
//! chksum-md5 = { version = "0.0.0", features = ["writer"] }
//! ```
//!
//! Alternatively, use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-md5 --features writer
//! ```
//!
//! # Example
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//! use std::io::Write; // required by writer
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let mut writer = md5::writer::new(file);
//!
//! writer.write_all(b"example data")?;
//!
//! let digest = writer.digest();
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```

use std::io::Write;

use chksum_writer as writer;
#[cfg(feature = "async-runtime-tokio")]
use tokio::io::AsyncWrite;

use crate::MD5;

/// A specialized [`Writer`](writer::Writer) type with the [`MD5`] hash algorithm.
pub type Writer<W> = writer::Writer<W, MD5>;

#[cfg(feature = "async-runtime-tokio")]
/// A specialized [`AsyncWriter`](writer::AsyncWriter) type with the [`MD5`] hash algorithm.
pub type AsyncWriter<R> = writer::AsyncWriter<R, MD5>;

/// Creates new [`Writer`].
pub fn new(inner: impl Write) -> Writer<impl Write> {
    writer::new(inner)
}

/// Creates new [`Writer`] with provided hash.
pub fn with_hash(inner: impl Write, hash: MD5) -> Writer<impl Write> {
    writer::with_hash(inner, hash)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`].
pub fn async_new(inner: impl AsyncWrite) -> AsyncWriter<impl AsyncWrite> {
    writer::async_new(inner)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`] with provided hash.
pub fn async_with_hash(inner: impl AsyncWrite, hash: MD5) -> AsyncWriter<impl AsyncWrite> {
    writer::async_with_hash(inner, hash)
}
