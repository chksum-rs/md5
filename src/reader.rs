//! This module is optional and can be enabled using the `reader` Cargo feature.
//!
//! The [`Reader`] allows on-the-fly calculation of the digest while reading the data.
//!
//! # Enabling
//!
//! Add the following entry to your `Cargo.toml` file to enable the `reader` feature:
//!
//! ```toml
//! [dependencies]
//! chksum-md5 = { version = "0.0.0", features = ["reader"] }
//! ```
//!
//! Alternatively, use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-md5 --features reader
//! ```
//!
//! # Example
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//! use std::io::Read; // required by reader
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let mut reader = md5::reader::new(file);
//!
//! let mut buffer = Vec::new();
//! reader.read_to_end(&mut buffer)?;
//! assert_eq!(buffer, b"example data");
//!
//! let digest = reader.digest();
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```

use std::io::Read;

use chksum_reader as reader;

use crate::MD5;

/// A specialized [`Reader`](reader::Reader) type with the [`MD5`] hash algorithm.
pub type Reader<R> = reader::Reader<R, MD5>;

/// Creates new [`Reader`].
pub fn new<R>(inner: R) -> Reader<R>
where
    R: Read,
{
    reader::new(inner)
}

/// Creates new [`Reader`] with provided hash.
pub fn with_hash<R>(inner: R, hash: MD5) -> Reader<R>
where
    R: Read,
{
    reader::with_hash(inner, hash)
}
