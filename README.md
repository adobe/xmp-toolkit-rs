# XMP Toolkit bindings for Rust

[![CI](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/xmp_toolkit.svg)](https://crates.io/crates/xmp_toolkit) [![codecov](https://codecov.io/gh/adobe/xmp-toolkit-rs/branch/main/graph/badge.svg?token=z1yA0Y6HZK)](https://codecov.io/gh/adobe/xmp-toolkit-rs)

Adobe's **[Extensible Metadata Platform (XMP)](https://www.adobe.com/devnet/xmp.html)** is a labeling technology that allows you to embed data about a file, known as metadata, into the file itself. More information on how partners and standards are using XMP is available at the [XMP website](https://www.adobe.com/products/xmp.html).

## Key features

This XMP Toolkit crate implements a subset of the XMP metadata standard. This toolkit allows a desktop or mobile application to:

* Parse XMP metadata found in many file formats.
* Inspect and modify the XMP data model.
* Embed and update XMP metadata in many file formats.

The [`XmpMeta` struct](https://docs.rs/xmp_toolkit/latest/xmp_toolkit/struct.XmpMeta.html) contains most of the API functions for these features and is the best place to get started in understanding this crate.

## Contributions and feedback

We welcome contributions to this project. For information on contributing, providing feedback, and about ongoing work, see [Contributing](./CONTRIBUTING.md).

## Requirements

The toolkit requires **Rust version 1.67.0** or newer. When a newer version of Rust becomes required, a new minor (1.x.0) version of this crate will be released.

## Crate features

This crate comes with the following features, which you can enable via your `Cargo.toml` file:

* `chrono` - When enabled, adds conversions between `XmpDateTime` and `chrono::DateTime<FixedOffset>`.
* `crt_static` - When enabled on Windows, uses the MSVC `/MT` build flag to request the static version of the C runtime instead of the dynamic version. This may help with avoiding conflicts with other libraries in the overall application. (This feature has no effect on any platform other than Windows.)

None of these features are enabled by default.

### Supported platforms

The toolkit has been tested on the following operating systems:

* Windows
  * Only the MSVC build chain is supported on Windows. As discussed in [#155](https://github.com/adobe/xmp-toolkit-rs/issues/155), we would welcome a PR to enable GNU build chain support on Windows.

* MacOS (Intel and Apple silicon)

* Ubuntu Linux on x86 and ARM v8 (aarch64)

## C++ XMP Toolkit

This crate incorporates the June 2022 version of the C++ [Adobe XMP Toolkit SDK](https://github.com/adobe/XMP-Toolkit-SDK/).

When a newer version of the C++ XMP Toolkit is incorporated, a new minor (1.x.0) version of this crate will be released.

## Upgrading from earlier versions

This API is considered to to be stable; in other words, no further breaking changes are anticipated. For instructions on how to upgrade from various 0.x versions to 1.x, see the [Upgrading guide](./UPGRADING.md).

Minor, non-breaking additions to the API surface may be added as the few remaining APIs in the C++ `XMP_Meta`, `XMP_Files`, and `TXMPUtils` interfaces are exposed. Such changes will trigger minor (1.x.0) version increments when they happen.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmp_toolkit = "1.6.0"
```

## License

The `xmp_toolkit` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates are licensed under different terms; please check the license terms for each crate and component for details.
