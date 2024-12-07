# chksum-md5

[![crates.io](https://img.shields.io/crates/v/chksum-md5?style=flat-square&logo=rust "crates.io")](https://crates.io/crates/chksum-md5)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/md5/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/md5/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum-md5?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum-md5/)
[![MSRV](https://img.shields.io/badge/MSRV-1.74.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/md5/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-md5/0.0.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-md5/0.0.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/md5?style=flat-square "LICENSE")](https://github.com/chksum-rs/md5/blob/master/LICENSE)

An implementation of the MD5 hash function with a straightforward interface for computing digests of bytes, files, directories, and more.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum-md5 = "0.0.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum-md5
```

## Usage

Use the `chksum` function to calculate digest of file, directory and so on.

```rust
use chksum_md5 as md5;

let file = File::open(path)?;
let digest = md5::chksum(file)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "5c71dbb287630d65ca93764c34d9aa0d"
);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/chksum-md5/).

## License

This crate is licensed under the MIT License.
