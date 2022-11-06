# XMP Toolkit bindings for Rust

[![CI](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/xmp_toolkit.svg)](https://crates.io/crates/xmp_toolkit) [![codecov](https://codecov.io/gh/adobe/xmp-toolkit-rs/branch/main/graph/badge.svg?token=z1yA0Y6HZK)](https://codecov.io/gh/adobe/xmp-toolkit-rs)

Adobe's **[Extensible Metadata Platform (XMP)](https://www.adobe.com/devnet/xmp.html)** is a labeling technology that allows you to embed data about a file, known as metadata, into the file itself. More information on how partners and standards are using XMP is available at the [XMP website](https://www.adobe.com/products/xmp.html).

## Key features

This XMP Toolkit crate implements a subset of the XMP metadata standard. This toolkit allows a desktop or mobile application to:

* Parse XMP metadata found in many file formats.
* Inspect and modify the XMP data model.
* Embed and update XMP metadata in many file formats.

## Contributions and feedback

We welcome contributions to this project. For information on contributing, providing feedback, and about ongoing work, see [Contributing](./CONTRIBUTING.md).

## Requirements

The toolkit requires **Rust version 1.56.0** or newer.

### Supported platforms

The toolkit has been tested on the following operating systems:

* Windows
* MacOS (Intel and Apple silicon)
* Ubuntu Linux

## C++ XMP Toolkit

This crate incorporates the June 2022 version of the C++ [Adobe XMP Toolkit SDK](https://github.com/adobe/XMP-Toolkit-SDK/).

When a newer version of the C++ XMP Toolkit is incorporated, a new minor (0.x.0) version of this crate will be released.

## Upgrading from earlier versions

**This version of the crate is a 1.0 release candidate. Please see [the API stabilization PR (#151)](https://github.com/adobe/xmp-toolkit-rs/pull/151) and add feedback there as appropriate.**

This API is considered to to be stable; in other words, no further breaking changes are anticipated. For instructions on how to upgrade from various 0.x versions to the current version, see the [Upgrading guide](./UPGRADING.md).

Minor, non-breaking additions to the API surface may be added as the few remaining APIs in the `XMP_Meta`, `XMP_Files`, `TXMPUtils` interfaces are exposed. Such changes will trigger minor (0.x.0) version increments when they happen.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmp_toolkit = "0.7.4"
```

## License

The `xmp_toolkit` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates are licensed under different terms; please check the license terms for each crate and component for details.
