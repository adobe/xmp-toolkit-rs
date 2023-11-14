# Changelog

All changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Do not manually edit this file. It will be automatically updated when a new release is published.

## 1.6.0
_14 November 2023_

* (MINOR) Bump MSRV to 1.67 ([#182](https://github.com/adobe/xmp-toolkit-rs/pull/182))

## 1.5.0
_16 October 2023_

* Use new Mac ARM runner in CI builds ([#180](https://github.com/adobe/xmp-toolkit-rs/pull/180))
* (MINOR) Bump MSRV to 1.66.0 ([#179](https://github.com/adobe/xmp-toolkit-rs/pull/179))
* (MINOR) Update num_enum requirement from 0.6.0 to 0.7.0 ([#178](https://github.com/adobe/xmp-toolkit-rs/pull/178))

## 1.4.0
_09 June 2023_

* (MINOR) Bump MSRV to 1.64.0 ([#176](https://github.com/adobe/xmp-toolkit-rs/pull/176))

## 1.3.0
_10 April 2023_

* Fix #135: `XmpMeta::from_str_with_options` returns empty data model if no `xmp_meta` wrapper exists ([#174](https://github.com/adobe/xmp-toolkit-rs/pull/174))
* (MINOR) Update to Rust 2021 edition ([#173](https://github.com/adobe/xmp-toolkit-rs/pull/173))
* Update num_enum requirement from 0.5.7 to 0.6.0 ([#169](https://github.com/adobe/xmp-toolkit-rs/pull/169))

## 1.2.1
_27 March 2023_

* README update for `crt_static` crate feature ([#167](https://github.com/adobe/xmp-toolkit-rs/pull/167))
* Added option for static crt using MT flag. ([#166](https://github.com/adobe/xmp-toolkit-rs/pull/166))
* Minor updates to documentation and doc links ([#168](https://github.com/adobe/xmp-toolkit-rs/pull/168))

## 1.2.0
_18 March 2023_

* (MINOR) Add crate feature to convert XmpDateTime to/from chrono DateTime<FixedOffset> ([#165](https://github.com/adobe/xmp-toolkit-rs/pull/165))

## 1.1.0
_10 March 2023_

* (MINOR) Add new `xmp_gps` module to help with Exif -> decimal conversions ([#163](https://github.com/adobe/xmp-toolkit-rs/pull/163))
* Require fs_extra 1.3 or newer ([#164](https://github.com/adobe/xmp-toolkit-rs/pull/164))

## 1.0.3
_07 March 2023_

* Officially support aarch64 on Linux ([#162](https://github.com/adobe/xmp-toolkit-rs/pull/162))
* CI: actions-rs/toolchain is no longer maintained; replace with dtolnay/rust-toolchain ([#161](https://github.com/adobe/xmp-toolkit-rs/pull/161))
* Fix aarch64 compilation issues ([#159](https://github.com/adobe/xmp-toolkit-rs/pull/159))

## 1.0.2
_09 February 2023_

* Implement Send for XmpMeta ([#158](https://github.com/adobe/xmp-toolkit-rs/pull/158))

## 1.0.1
_31 January 2023_

* Add a note to README stating that only MSVC build chain is supported currently on Windows
* Don't call exit(1) if unable to initialize C++ XMP Toolkit ([#156](https://github.com/adobe/xmp-toolkit-rs/pull/156))

## 1.0.0
_19 December 2022_

* (MAJOR) Prepare for stable API (1.0) release ([#151](https://github.com/adobe/xmp-toolkit-rs/pull/151))
* Fix new Clippy warning in Rust 1.66 ([#154](https://github.com/adobe/xmp-toolkit-rs/pull/154))

## 0.7.6
_06 November 2022_

* Review API documentation ([#153](https://github.com/adobe/xmp-toolkit-rs/pull/153))
* Add `XmpMeta::sort` ([#152](https://github.com/adobe/xmp-toolkit-rs/pull/152))

## 0.7.5
_06 November 2022_

* Add reference to API stabilization PR
* New README; propose 1.0 release candidate status
* Port XMP_CoreCoverage.cpp test suite ([#108](https://github.com/adobe/xmp-toolkit-rs/pull/108))
* Add `XmpDateTime::convert_to_local_time` and `XmpDateTime::convert_to_utc` ([#149](https://github.com/adobe/xmp-toolkit-rs/pull/149))
* Add `XmpDateTime::set_local_time_zone` ([#148](https://github.com/adobe/xmp-toolkit-rs/pull/148))
* Add `XmpMeta::compose_field_selector` ([#147](https://github.com/adobe/xmp-toolkit-rs/pull/147))
* Add `XmpMeta::compose_lang_selector` ([#146](https://github.com/adobe/xmp-toolkit-rs/pull/146))

## 0.7.4
_04 November 2022_

* Add APIs to allow iteration over the XMP data model ([#141](https://github.com/adobe/xmp-toolkit-rs/pull/141))

## 0.7.3
_03 November 2022_

* Fix misplaced reference to WXMPIterator.cpp ([#139](https://github.com/adobe/xmp-toolkit-rs/pull/139))
* If property value is empty string, pass it to C++ toolkit as NULL ([#138](https://github.com/adobe/xmp-toolkit-rs/pull/138))
* Replace `XmpMeta::from_str_requiring_xmp_meta` with `XmpMeta::from_str_with_options` ([#137](https://github.com/adobe/xmp-toolkit-rs/pull/137))
* Add `XmpMeta::from_str_requiring_xmp_meta` ([#136](https://github.com/adobe/xmp-toolkit-rs/pull/136))

## 0.7.2
_01 November 2022_

* Add `XmpMeta::set_localized_text` ([#133](https://github.com/adobe/xmp-toolkit-rs/pull/133))
* Add `XmpMeta::delee_qualifier` ([#132](https://github.com/adobe/xmp-toolkit-rs/pull/132))
* Add `XmpMeta::delete_struct_field` ([#131](https://github.com/adobe/xmp-toolkit-rs/pull/131))
* Add `XmpMeta::delete_array_item` ([#130](https://github.com/adobe/xmp-toolkit-rs/pull/130))
* Add `XmpMeta::contains_qualifier` ([#129](https://github.com/adobe/xmp-toolkit-rs/pull/129))
* Add `XmpMeta::qualifier` ([#128](https://github.com/adobe/xmp-toolkit-rs/pull/128))
* Allow `XmpMeta::array_item` to accept `XmpMeta::LAST_ITEM` ([#127](https://github.com/adobe/xmp-toolkit-rs/pull/127))
* Add `XmpMeta::array_item` ([#126](https://github.com/adobe/xmp-toolkit-rs/pull/126))
* Add `XmpMeta::delete_property` ([#125](https://github.com/adobe/xmp-toolkit-rs/pull/125))
* Add `XmpMeta::set_qualifier` ([#124](https://github.com/adobe/xmp-toolkit-rs/pull/124))
* Add `XmpMeta::compose_qualifier_path` ([#123](https://github.com/adobe/xmp-toolkit-rs/pull/123))
* Add `XmpMeta::array_len` ([#122](https://github.com/adobe/xmp-toolkit-rs/pull/122))
* Add `XmpMeta::set_array_item` ([#121](https://github.com/adobe/xmp-toolkit-rs/pull/121))
* Implement `Display` for `XmpMeta` ([#120](https://github.com/adobe/xmp-toolkit-rs/pull/120))
* Add `XmpMeta::compose_array_item_path` ([#119](https://github.com/adobe/xmp-toolkit-rs/pull/119))
* Add `XmpMeta::set_struct_field` ([#118](https://github.com/adobe/xmp-toolkit-rs/pull/118))
* Add `XmpMeta::append_array_item` ([#117](https://github.com/adobe/xmp-toolkit-rs/pull/117))
* Add `XmpMeta::namespace_prefix` and `XmpMeta::namespace_uri` accessors ([#116](https://github.com/adobe/xmp-toolkit-rs/pull/116))

## 0.7.1
_24 October 2022_

* Implement `Clone` for `XmpMeta` ([#114](https://github.com/adobe/xmp-toolkit-rs/pull/114))
* Implement `Default` for `XmpMeta` ([#113](https://github.com/adobe/xmp-toolkit-rs/pull/113))
* Add `XmpMeta::name` and `XmpMeta::set_name` accessors ([#112](https://github.com/adobe/xmp-toolkit-rs/pull/112))
* Impl `Debug` for `XmpMeta` ([#111](https://github.com/adobe/xmp-toolkit-rs/pull/111))
* Fix crash if `XmpMeta::debug_dump_namespaces` was the first call into XMP Toolkit ([#110](https://github.com/adobe/xmp-toolkit-rs/pull/110))
* Add `XmpMeta::debug_dump_namespaces()` ([#109](https://github.com/adobe/xmp-toolkit-rs/pull/109))

## 0.7.0
_23 October 2022_

* Add example (port of MyReadXMP app) ([#82](https://github.com/adobe/xmp-toolkit-rs/pull/82))
* Add `XmpMeta::struct_field` ([#107](https://github.com/adobe/xmp-toolkit-rs/pull/107))
* Add `XmpMeta::contains_struct_field` ([#106](https://github.com/adobe/xmp-toolkit-rs/pull/106))
* Add `XmpMeta::compose_struct_field_path` ([#105](https://github.com/adobe/xmp-toolkit-rs/pull/105))
* Fix memory leaks when returning strings from C++ to Rust ([#104](https://github.com/adobe/xmp-toolkit-rs/pull/104))
* Implement `Display` for `XmpDateTime` ([#103](https://github.com/adobe/xmp-toolkit-rs/pull/103))
* Add `XmpMeta::localized_text` accessor ([#99](https://github.com/adobe/xmp-toolkit-rs/pull/99))
* Refactor `XmpMeta` documentation for property accessors ([#98](https://github.com/adobe/xmp-toolkit-rs/pull/98))
* (MINOR) Rename `XmpMeta::does_property_exist` to `XmpMeta::contains_property` ([#97](https://github.com/adobe/xmp-toolkit-rs/pull/97))
* (MINOR) Rename `XmpMeta.array_property` to `.property_array` ([#96](https://github.com/adobe/xmp-toolkit-rs/pull/96))
* Add typed property setters ([#95](https://github.com/adobe/xmp-toolkit-rs/pull/95))
* Add typed property getters ([#94](https://github.com/adobe/xmp-toolkit-rs/pull/94))
* Remove `Eq` trait bound on `XmpValue<T>` ([#93](https://github.com/adobe/xmp-toolkit-rs/pull/93))

## 0.6.0
_20 October 2022_

* (MINOR) `XmpMeta::set_property` can now pass `XmpValue` options ([#88](https://github.com/adobe/xmp-toolkit-rs/pull/88))
* Add options for setting flags on `XmpValue` ([#87](https://github.com/adobe/xmp-toolkit-rs/pull/87))
* (MINOR) Rework `XmpDateTime` as a non-opaque type ([#86](https://github.com/adobe/xmp-toolkit-rs/pull/86))
* (MINOR) Refactor how property values are returned from accessor functions ([#81](https://github.com/adobe/xmp-toolkit-rs/pull/81))
* Add new API `XmpIter::array_property` ([#78](https://github.com/adobe/xmp-toolkit-rs/pull/78))
* Add new API function `XmpMeta::from_str` ([#77](https://github.com/adobe/xmp-toolkit-rs/pull/77))
* (MINOR) Bump MSRV to 1.56 ([#75](https://github.com/adobe/xmp-toolkit-rs/pull/75))

## 0.5.3
_20 July 2022_

* Remove restriction on Unicode license; allows unicode-ident version to float ([#73](https://github.com/adobe/xmp-toolkit-rs/pull/73))

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
