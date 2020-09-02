# XMP Toolkit bindings for Rust

[![Crate](https://img.shields.io/crates/v/xmp_toolkit.svg)](https://crates.io/crates/xmp_toolkit)
[![API](https://docs.rs/xmp_toolkit/badge.svg)](https://docs.rs/xmp_toolkit)
[![Tests](https://github.com/adobe/xmp-toolkit-rs/workflows/Tests/badge.svg)](https://github.com/adobe/xmp-toolkit-rs/actions?query=workflow%3ATests)

This crate provides a binding of the [Adobe XMP Toolkit SDK](https://github.com/adobe/XMP-Toolkit-SDK/) to Rust.

The portions of the binding that are present are believed to be well-tested and correct, but it is not (as of yet) a complete binding.

Contributions that ...

* extend the supported platforms (current Mac and Linux only)
* extend the supported API surface

... are especially welcomed. Please read the [Contributing Guide](./CONTRIBUTING.md) for more information.

## Rust Language Support

As of this writing, this crate requires **Rust version 1.44** or newer. (The CI builds use this version of Rust.) This may be increased to a newer version at any time, but will be noted in the changelog.

This crate follows all of the typical Rust conventions (`cargo build`, `cargo test`, etc.). There is a `build.rs` script which will ensure that the C++ portions of the library are built as needed. It may need to be updated for platforms that haven't already been tested.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmp_toolkit = "0.1"
```

## License

This project is licensed under the Apache V2 License or the MIT License, at your option. See the [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) files for more information.
