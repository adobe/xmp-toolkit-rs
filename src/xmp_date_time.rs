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

// TO DO: Revise API documentation to fit the Rust wrapper.

use crate::ffi;

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
    /// Creates a new file struct that is associated with no file.
    pub fn new() -> XmpDateTime {
        XmpDateTime {
            dt: unsafe { ffi::CXmpDateTimeNew() },
        }
    }

    pub fn current() -> XmpDateTime {
        XmpDateTime {
            dt: unsafe { ffi::CXmpDateTimeCurrent() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let mut _dt = XmpDateTime::new();
    }

    #[test]
    fn current() {
        let mut _dt = XmpDateTime::current();
    }
}
