# Guide to upgrading from 0.x versions

## Upgrading to 0.7 from earlier versions

The `XmpMeta::array_property` method has been renamed to `XmpMeta::property_array`
to make it consistent with the other typed property getters.

The `XmpMeta::does_property_exist` method has been renamed to `XmpMeta::contains_property`
for consistency with other Rust container types.

## Upgrading to 0.6 from earlier versions

The `XmpMeta::property` method has been changed to return `Option<XmpValue<String>>`
instead of `Option<String>`. You may need to add a `.value` dereference to get the
string value from existing calls to the `property` accessor. The XMP value flags
(known as `XMP_OptionBits` in the C++ XMP Toolkit) are now available via accessors
on the new `XmpValue` struct.

The `XmpMeta::set_property` and `XmpMeta::set_property_date` methods have been changed
to require `XmpValue<String>` and `XmpValue<XmpDateTime>`, respectively. This allows
you to pass XMP value flags when setting values. `XmpValue<T>` implements `From<T>`,
so in most cases, the default/pre-existing behavior can be retained by adding `.into()`
at the call sites.

The `XmpDateTime` struct has been meaningfully implemented, meaning it has changed
from an opaque type to a struct containing the date, time, and time zone values as
present in the C++ toolkit.

This version also increases the minimum supported Rust version (MSRV) to 1.56.0.

## Upgrading to 0.5 from earlier releases

Prior versions of the Rust XMP Toolkit mostly ignored the possibility that the C++ XMP Toolkit could throw exceptions. Among other things, this created the possibility of unexpected behavior if the C++ runtime attempted to unwind the stack through Rust code.

This version introduces `XmpError` and `XmpResult` types which mirror the information from the underlying C++ `XMP_Error` type and retrofits existing APIs to use them appropriately. (A few APIs which returned `Option<...>` were left unchanged; those APIs now map error conditions to a `None` response.)

## Upgrading to 0.4 from earlier releases

The `xmp_const` module has been removed and a new `xmp_ns` module has been added, containing constants for many common XMP namespaces. Replace `xmp_const::XMP_NS_XMP` with `xmp_ns::XMP`.

The `OpenFileOptions` mod has been reworked as an opaque type, removing the need for the bitflags crate dependency. Create by using `OpenFileOptions::default()` and then calling methods on the struct to add options as needed.
