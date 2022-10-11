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
mod xmp_meta;
pub mod xmp_ns;

pub use xmp_date_time::XmpDateTime;
pub use xmp_error::{XmpError, XmpErrorType, XmpResult};
pub use xmp_file::{OpenFileOptions, XmpFile};
pub use xmp_meta::{ArrayProperty, XmpMeta, XmpValue};

#[cfg(test)]
mod tests;
