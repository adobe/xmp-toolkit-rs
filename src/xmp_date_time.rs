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

use crate::ffi;

/// The expanded type for a date and time.
///
/// Dates and time in the serialized XMP are ISO 8601 strings.
/// The `XmpDateTime` struct allows easy conversion with other formats.
pub struct XmpDateTime {
    pub(crate) dt: *mut ffi::CXmpDateTime,
}

impl Drop for XmpDateTime {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpDateTimeDrop(self.dt);
        }
    }
}

impl Default for XmpDateTime {
    fn default() -> Self {
        XmpDateTime::new()
    }
}

impl XmpDateTime {
    /// Creates a new date-time struct with zeros in all fields.
    pub fn new() -> XmpDateTime {
        XmpDateTime {
            dt: unsafe { ffi::CXmpDateTimeNew() },
        }
    }

    /// Creates a new date-time struct reflecting the current time.
    pub fn current() -> XmpDateTime {
        XmpDateTime {
            dt: unsafe { ffi::CXmpDateTimeCurrent() },
        }
    }
}
