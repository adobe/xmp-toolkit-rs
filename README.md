# XMP Toolkit bindings for Rust

[![CI](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/xmp_toolkit.svg)](https://crates.io/crates/xmp_toolkit) [![codecov](https://codecov.io/gh/adobe/xmp-toolkit-rs/branch/main/graph/badge.svg?token=z1yA0Y6HZK)](https://codecov.io/gh/adobe/xmp-toolkit-rs)

This crate provides a binding of the [Adobe XMP Toolkit SDK](https://github.com/adobe/XMP-Toolkit-SDK/) to Rust.

The portions of the binding that are present are believed to be well-tested and correct, but it is not (as of yet) a complete binding.

Contributions that ...

* extend the supported platforms (currently Windows, Mac, and Linux only)
* extend the supported API surface

... are especially welcomed. Please read the [Contributing Guide](./CONTRIBUTING.md) for more information.

## Rust language support

As of this writing, this crate requires **Rust version 1.54** or newer. (The CI builds use this version of Rust.) This may be increased to a newer version at any time, but will be noted in the changelog.

This crate follows all of the typical Rust conventions (`cargo build`, `cargo test`, etc.). There is a `build.rs` script which will ensure that the C++ portions of the library are built as needed. It may need to be updated for platforms that haven't already been tested.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmp_toolkit = "0.3.8"
```

## Breaking changes in 0.x series

### Upgrading to 0.4 from earlier releases

The `xmp_const` module has been removed and a new `xmp_ns` module has been added, containing constants for many common XMP namespaces. Replace `xmp_const::XMP_NS_XMP` with `xmp_ns::XMP`.

The `OpenFileOptions` mod has been reworked as an opaque type, removing the need for the bitflags crate dependency. Create by using `OpenFileOptions::default()` and then calling methods on the struct to add options as needed.

## License

The `xmp_toolkit` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates are licensed under different terms; please check the license terms for each crate and component for details.
