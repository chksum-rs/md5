//! This crate provides an implementation of the MD5 hash function with a straightforward interface for computing digests of bytes, files, directories, and more.
//!
//! For a low-level interface, you can explore the [`chksum_hash_md5`] crate.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-md5 = "0.1.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-md5
//! ```     
//!
//! # Usage
//!
//! Use the [`chksum`] function to calculate digest of file, directory and so on.
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = md5::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Asynchronous Runtime
//!
//! Use the [`async_chksum`] function to calculate digest of file, directory and so on.
//!
//! ```rust
//! # #[cfg(feature = "async-runtime-tokio")]
//! # {
//! # use std::path::Path;
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//! use tokio::fs::File;
//!
//! # async fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path).await?;
//! let digest = md5::async_chksum(file).await?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! # Input Types
//!
//! ## Bytes
//!
//! ### Array
//!
//! ```rust
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let data = [0, 1, 2, 3];
//! let digest = md5::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Vec
//!
//! ```rust
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let data = vec![0, 1, 2, 3];
//! let digest = md5::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Slice
//!
//! ```rust
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let data = &[0, 1, 2, 3];
//! let digest = md5::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Strings
//!
//! ### str
//!
//! ```rust
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let data = "&str";
//! let digest = md5::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### String
//!
//! ```rust
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let data = String::from("String");
//! let digest = md5::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## File
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = md5::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Directory
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::read_dir;
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let readdir = read_dir(path)?;
//! let digest = md5::chksum(readdir)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Path
//!
//! ```rust
//! # use std::path::Path;
//! use std::path::PathBuf;
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let path = PathBuf::from(path);
//! let digest = md5::chksum(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Standard Input
//!
//! ```rust
//! use std::io::stdin;
//!
//! # use chksum_md5::Result;
//! use chksum_md5 as md5;
//!
//! # fn wrapper() -> Result<()> {
//! let stdin = stdin();
//! let digest = md5::chksum(stdin)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "5c71dbb287630d65ca93764c34d9aa0d"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! Cargo features are utilized to enable extra options.
//!
//! * `reader` enables the [`reader`] module with the [`Reader`] struct.
//! * `writer` enables the [`writer`] module with the [`Writer`] struct.
//!
//! By default, neither of these features is enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum-md5 = { version = "0.1.0", features = ["reader", "writer"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-md5 --features reader,writer
//! ```
//!
//! ## Asynchronous Runtime
//!
//! * `async-runtime-tokio`: Enables async interface for Tokio runtime.
//!
//! By default, neither of these features is enabled.
//!
//! # Disclaimer
//!
//! The MD5 hash function should be used only for backward compatibility due to security issues.
//!
//! Check [RFC 6151: Updated Security Considerations for the MD5 Message-Digest and the HMAC-MD5 Algorithms](https://www.rfc-editor.org/rfc/rfc6151) for more details.
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

#[cfg(feature = "reader")]
pub mod reader;
#[cfg(feature = "writer")]
pub mod writer;

use std::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use chksum_core as core;
#[cfg(feature = "async-runtime-tokio")]
#[doc(no_inline)]
pub use chksum_core::AsyncChksumable;
#[doc(no_inline)]
pub use chksum_core::{Chksumable, Error, Hash, Hashable, Result};
#[doc(no_inline)]
pub use chksum_hash_md5 as hash;

#[cfg(all(feature = "reader", feature = "async-runtime-tokio"))]
#[doc(inline)]
pub use crate::reader::AsyncReader;
#[cfg(feature = "reader")]
#[doc(inline)]
pub use crate::reader::Reader;
#[cfg(all(feature = "writer", feature = "async-runtime-tokio"))]
#[doc(inline)]
pub use crate::writer::AsyncWriter;
#[cfg(feature = "writer")]
#[doc(inline)]
pub use crate::writer::Writer;

/// Creates a new hash.
///
/// # Example
///
/// ```rust
/// use chksum_md5 as md5;
///
/// let mut hash = md5::new();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "5c71dbb287630d65ca93764c34d9aa0d"
/// );
/// ```
#[must_use]
pub fn new() -> MD5 {
    MD5::new()
}

/// Creates a default hash.
///
/// # Example
///
/// ```rust
/// use chksum_md5 as md5;
///
/// let mut hash = md5::default();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "5c71dbb287630d65ca93764c34d9aa0d"
/// );
/// ```
#[must_use]
pub fn default() -> MD5 {
    core::default()
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_md5 as md5;
///
/// let data = b"example data";
/// let digest = md5::hash(data);
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "5c71dbb287630d65ca93764c34d9aa0d"
/// );
/// ```
pub fn hash(data: impl core::Hashable) -> Digest {
    core::hash::<MD5>(data)
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_md5 as md5;
///
/// let data = b"example data";
/// if let Ok(digest) = md5::chksum(data) {
///     assert_eq!(
///         digest.to_hex_lowercase(),
///         "5c71dbb287630d65ca93764c34d9aa0d"
///     );
/// }
/// ```
pub fn chksum(data: impl core::Chksumable) -> Result<Digest> {
    core::chksum::<MD5>(data)
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_md5 as md5;
///
/// # async fn wrapper() {
/// let data = b"example data";
/// if let Ok(digest) = md5::async_chksum(data).await {
///     assert_eq!(
///         digest.to_hex_lowercase(),
///         "5c71dbb287630d65ca93764c34d9aa0d"
///     );
/// }
/// # }
/// ```
#[cfg(feature = "async-runtime-tokio")]
pub async fn async_chksum(data: impl core::AsyncChksumable) -> Result<Digest> {
    core::async_chksum::<MD5>(data).await
}

/// The MD5 hash instance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MD5 {
    inner: hash::Update,
}

impl MD5 {
    /// Calculates the hash digest of an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5::MD5;
    ///
    /// let data = b"example data";
    /// let digest = MD5::hash(data);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "5c71dbb287630d65ca93764c34d9aa0d"
    /// );
    /// ```
    #[must_use]
    pub fn hash<T>(data: T) -> Digest
    where
        T: AsRef<[u8]>,
    {
        let mut hash = Self::new();
        hash.update(data);
        hash.digest()
    }

    /// Creates a new hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5::MD5;
    ///
    /// let mut hash = MD5::new();
    /// hash.update(b"example data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "5c71dbb287630d65ca93764c34d9aa0d"
    /// );
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let inner = hash::Update::new();
        Self { inner }
    }

    /// Updates the hash state with an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5::MD5;
    ///
    /// let mut hash = MD5::new();
    /// hash.update(b"example");
    /// hash.update(" ");
    /// hash.update("data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "5c71dbb287630d65ca93764c34d9aa0d"
    /// );
    /// ```
    pub fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.inner.update(data);
    }

    /// Resets the hash state to its initial state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5::MD5;
    ///
    /// let mut hash = MD5::new();
    /// hash.update(b"example data");
    /// hash.reset();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d41d8cd98f00b204e9800998ecf8427e"
    /// );
    /// ```
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Produces the hash digest.
    ///
    /// # Example
    ///
    /// ```
    /// use chksum_md5::MD5;
    ///
    /// let mut hash = MD5::new();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d41d8cd98f00b204e9800998ecf8427e"
    /// );
    /// ```
    #[must_use]
    pub fn digest(&self) -> Digest {
        self.inner.digest().into()
    }
}

impl core::Hash for MD5 {
    type Digest = Digest;

    fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.update(data);
    }

    fn reset(&mut self) {
        self.reset();
    }

    fn digest(&self) -> Self::Digest {
        self.digest()
    }
}

/// A hash digest.
pub struct Digest(hash::Digest);

impl Digest {
    /// Creates a new digest.
    #[must_use]
    pub const fn new(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        let inner = hash::Digest::new(digest);
        Self(inner)
    }

    /// Returns a byte slice of the digest's contents.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }

    /// Consumes the digest, returning the digest bytes.
    #[must_use]
    pub fn into_inner(self) -> [u8; hash::DIGEST_LENGTH_BYTES] {
        let Self(inner) = self;
        inner.into_inner()
    }

    /// Returns a string in the lowercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5 as md5;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xD4, 0x1D, 0x8C, 0xD9,
    ///     0x8F, 0x00, 0xB2, 0x04,
    ///     0xE9, 0x80, 0x09, 0x98,
    ///     0xEC, 0xF8, 0x42, 0x7E,
    /// ];
    /// let digest = md5::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d41d8cd98f00b204e9800998ecf8427e"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_lowercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_lowercase()
    }

    /// Returns a string in the uppercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_md5 as md5;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xD4, 0x1D, 0x8C, 0xD9,
    ///     0x8F, 0x00, 0xB2, 0x04,
    ///     0xE9, 0x80, 0x09, 0x98,
    ///     0xEC, 0xF8, 0x42, 0x7E,
    /// ];
    /// let digest = md5::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_uppercase(),
    ///     "D41D8CD98F00B204E9800998ECF8427E"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_uppercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_uppercase()
    }
}

impl core::Digest for Digest {}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        Display::fmt(inner, f)
    }
}

impl LowerHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        LowerHex::fmt(inner, f)
    }
}

impl UpperHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        UpperHex::fmt(inner, f)
    }
}

impl From<[u8; hash::DIGEST_LENGTH_BYTES]> for Digest {
    fn from(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        Self::new(digest)
    }
}

impl From<hash::Digest> for Digest {
    fn from(digest: hash::Digest) -> Self {
        Self(digest)
    }
}
