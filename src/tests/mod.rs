// Copyright 2022 Adobe. All rights reserved.
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

// Tests are grouped under this module so as to avoid
// having the test code itself included in coverage numbers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

mod fixtures;
mod xmp_core_coverage;
mod xmp_date_time;
#[cfg(feature = "chrono")]
mod xmp_date_time_chrono;
mod xmp_error;
mod xmp_error_type;
mod xmp_file;
mod xmp_gps;
mod xmp_iterator;
mod xmp_meta;
mod xmp_value;
