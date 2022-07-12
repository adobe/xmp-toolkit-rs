# XMP Toolkit bindings for Rust

[![CI](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adobe/xmp-toolkit-rs/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/xmp_toolkit.svg)](https://crates.io/crates/xmp_toolkit) [![codecov](https://codecov.io/gh/adobe/xmp-toolkit-rs/branch/main/graph/badge.svg?token=z1yA0Y6HZK)](https://codecov.io/gh/adobe/xmp-toolkit-rs)

This crate provides a binding of the [Adobe XMP Toolkit SDK](https://github.com/adobe/XMP-Toolkit-SDK/) to Rust.

The portions of the binding that are present are believed to be well-tested and correct, but it is not (as of yet) a complete binding.

Contributions that ...

* extend the supported platforms (see _Officially-supported platforms_ below)
* extend the supported API surface

... are especially welcomed. Please read the [Contributing Guide](./CONTRIBUTING.md) for more information.

## Rust language support

As of this writing, this crate requires **Rust version 1.54** or newer. (The CI builds use this version of Rust.) This may be increased to a newer version at any time, but will be noted in the changelog.

This crate follows all of the typical Rust conventions (`cargo build`, `cargo test`, etc.). There is a `build.rs` script which will ensure that the C++ portions of the library are built as needed. It may need to be updated for platforms that haven't already been tested.

## Officially-supported platforms

The following platforms are officially supported and tested.

* Windows latest (64-bit Intel)
* MacOS latest (64-bit Intel)
* Ubuntu latest (64-bit Intel)

All pull requests are validated via [GitHub-hosted runners on GitHub Actions](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners) and must pass validation on each platform before being merged. These use the `latest` version of each OS as defined by GitHub at the time.

## Unofficially-supported platforms

The following platforms are believed and intended to work, but are not officially supported:

* MacOS latest (64-bit ARM)
* Ubuntu latest (64-bit ARM)

GitHub does not currently provide GitHub-hosted runners for these platforms and our team has not arranged for self-hosted runners at this time. Since we can not validate each pull request via validation, there is a risk of undetected breakage for any release.

We welcome bug reports and PRs regarding build issues on these and other platforms and will address them to the best of our ability.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xmp_toolkit = "0.5.0"
```

## Breaking changes in 0.x series

### Upgrading to 0.5 from earlier releases

Prior versions of the Rust XMP Toolkit mostly ignored the possibility that the C++ XMP Toolkit could throw exceptions. Among other things, this created the possibility of unexpected behavior if the C++ runtime attempted to unwind the stack through Rust code.

This version introduces `XmpError` and `XmpResult` types which mirror the information from the underlying C++ `XMP_Error` type and retrofits existing APIs to use them appropriately. (A few APIs which returned `Option<...>` were left unchanged; those APIs now map error conditions to a `None` response.)

### Upgrading to 0.4 from earlier releases

The `xmp_const` module has been removed and a new `xmp_ns` module has been added, containing constants for many common XMP namespaces. Replace `xmp_const::XMP_NS_XMP` with `xmp_ns::XMP`.

The `OpenFileOptions` mod has been reworked as an opaque type, removing the need for the bitflags crate dependency. Create by using `OpenFileOptions::default()` and then calling methods on the struct to add options as needed.

## License

The `xmp_toolkit` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates are licensed under different terms; please check the license terms for each crate and component for details.
