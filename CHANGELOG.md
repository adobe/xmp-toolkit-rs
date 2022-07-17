# Changelog

All changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Do not manually edit this file. It will be automatically updated when a new release is published.

## 0.5.2
_17 July 2022_

* Deny panic in production code ([#70](https://github.com/adobe/xmp-toolkit-rs/pull/70))
* Introduce new error type `XmpErrorType::NulInRustString` ([#68](https://github.com/adobe/xmp-toolkit-rs/pull/68))
* Fix unicode-ident to 1.0.1 until review of additional license ([#69](https://github.com/adobe/xmp-toolkit-rs/pull/69))

## 0.5.1
_14 July 2022_

* Fix broken documentation build ([#67](https://github.com/adobe/xmp-toolkit-rs/pull/67))

## 0.5.0
_12 July 2022_

* (MINOR) Pass XMP errors from C++ to Rust ([#60](https://github.com/adobe/xmp-toolkit-rs/pull/60))
* Refactor test code so it can be excluded from code coverage ([#66](https://github.com/adobe/xmp-toolkit-rs/pull/66))
* Require code coverage to upload on all supported platforms ([#63](https://github.com/adobe/xmp-toolkit-rs/pull/63))
* (MINOR) Hide access to FFI functions ([#61](https://github.com/adobe/xmp-toolkit-rs/pull/61))

## 0.4.0
_07 July 2022_

* Fixed build failures on ARM Linux platforms ([#57](https://github.com/adobe/xmp-toolkit-rs/pull/57))
* Silence C++ compiler warnings on Mac ([#55](https://github.com/adobe/xmp-toolkit-rs/pull/55))
* Rework `OpenFileOptions` as an opaque type ([#54](https://github.com/adobe/xmp-toolkit-rs/pull/54))
* (MINOR) New module for XMP namespace constants ([#53](https://github.com/adobe/xmp-toolkit-rs/pull/53))
* Require API docs for all public API surfaces ([#51](https://github.com/adobe/xmp-toolkit-rs/pull/51))
* (MINOR) Bump MSRV to 1.54 ([#52](https://github.com/adobe/xmp-toolkit-rs/pull/52))
* Improve docs for `XmpMeta` ([#50](https://github.com/adobe/xmp-toolkit-rs/pull/50))
* Improve docs for `XmpFile` ([#49](https://github.com/adobe/xmp-toolkit-rs/pull/49))
* Configure dependabot to watch dependencies brought in via git submodules ([#48](https://github.com/adobe/xmp-toolkit-rs/pull/48))
* Add `impl std::error::Error` for `XmpFileError` ([#47](https://github.com/adobe/xmp-toolkit-rs/pull/47))

## 0.3.8
_22 June 2022_

* Add convenience function for reading XMP from a file ([#46](https://github.com/adobe/xmp-toolkit-rs/pull/46))
* Update XMP Toolkit to June 2022 release ([#42](https://github.com/adobe/xmp-toolkit-rs/pull/42))
* Update libexpat to v2.4.8 ([#41](https://github.com/adobe/xmp-toolkit-rs/pull/41))

## 0.3.7
_18 June 2022_

* Fix publish workflow to include git submodules

## 0.3.6 (YANKED because the release was incomplete)
_17 June 2022_

* Use cargo publish --no-verify because repo contents must be modified during build

## 0.3.5 (NOT RELEASED on crates.io due to bug in publish workflow)
_17 June 2022_

* Improve build infrastructure ([#44](https://github.com/adobe/xmp-toolkit-rs/pull/44))

## 0.3.4
_28 April 2022_

* Fix a subtle bug in converting to modern function prototypes.

## 0.3.3
_28 April 2022_

* Update zlib to v1.2.12. (#38)
* Add Rust 1.60.0 to CI build matrix.
* Remove Rust 1.59.0 from CI build matrix.

## 0.3.2
_08 March 2022_

* Update XMP Toolkit to February 2022 release. (#35)
* Update libexpat to v2.4.6. (#36)
* Add Rust 1.59.0 to CI build matrix.
* Remove Rust 1.58.0 from CI build matrix.

## 0.3.1
_28 January 2022_

* Implement `Display` trait in `XmpFileError` enum. (#32)
* Fix Clippy warning about `assert_eq!` with a bool value. (#33)
* Add Rust 1.58.0 to CI build matrix. (#30)
* Remove Rust 1.56.0 from CI build matrix.

## 0.3.0
_27 October 2021_

* Add Rust 1.56.0 to CI build matrix. (#29)
* Remove Rust 1.53.0 from CI build matrix.
* Update XMP Toolkit submodule to October 2021 Release. (#28)

## 0.2.0
_12 October 2021_

* Update XMP Toolkit submodule to August 2021 Release.
* Bumped MSRV to 1.46.0 due to new dependency from bitflags crate.
* Add Rust 1.55.0 to CI build matrix.
* Remove Rust 1.53.0 from CI build matrix.
* Update version references in Cargo.toml.

## 0.1.8
_23 June 2021_

* Include libexpat via git submodule and update to version 2.4.1. (#18)
* Add Rust 1.53.0 to CI build matrix.
* Remove Rust 1.51.0 from CI build matrix.

## 0.1.7
_29 March 2021_

* Add Rust 1.51.0 to CI build matrix.
* Remove Rust 1.48.0 from CI build matrix.

## 0.1.6
_25 November 2020_

* Handle exceptions on OpenFile. (#13)
* Add Rust 1.48.0 to CI build matrix.
* Remove Rust 1.47.0 from CI build matrix.

## 0.1.5
_12 October 2020_

* Add support for building on Windows.
* Add Rust 1.47.0 to CI build matrix.
* Remove Rust 1.46.0 from CI build matrix.

## 0.1.4
_04 September 2020_

* Add Mac OS builds to CI build matrix.
* Add Rust 1.46.0 to CI build matrix.

## 0.1.3
_02 September 2020_

* Improve API documentation.

## 0.1.2
_01 September 2020_

* Fix another build issue that occurs only within the docs.rs environment.

## 0.1.1
_01 September 2020_

* Fix build issue that occurs within the docs.rs environment.

## 0.1.0
_01 September 2020_

* **Initial public release.**
  * Supports Rust 1.44.0 and higher.
  * Includes the 24 July 2020 release of C++ XMP Toolkit SDK.
