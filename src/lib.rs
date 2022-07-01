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

#![deny(warnings)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod ffi;

mod xmp_const;
pub use xmp_const::*;

mod xmp_date_time;
pub use xmp_date_time::XmpDateTime;

mod xmp_file;
pub use xmp_file::OpenFileOptions;
pub use xmp_file::XmpFile;
pub use xmp_file::XmpFileError;

mod xmp_meta;
pub use xmp_meta::XmpMeta;
