// Copyright 2020 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![deny(warnings)]
#![doc = include_str!("../README.md")]

mod ffi;
mod xmp_date_time;
mod xmp_error;
mod xmp_file;
pub mod xmp_gps;
mod xmp_iterator;
mod xmp_meta;
pub mod xmp_ns;
mod xmp_value;

#[cfg(feature = "chrono")]
pub use xmp_date_time::DateTimeConvertError;
pub use xmp_date_time::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};
pub use xmp_error::{XmpError, XmpErrorType, XmpResult};
pub use xmp_file::{OpenFileOptions, XmpFile};
pub use xmp_iterator::{IterOptions, XmpIterator, XmpProperty};
pub use xmp_meta::{ArrayProperty, FromStrOptions, ItemPlacement, ToStringOptions, XmpMeta};
pub use xmp_value::XmpValue;

#[cfg(test)]
mod tests;
