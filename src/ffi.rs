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

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

pub(crate) struct CXmpString {
    pub(crate) s: *const c_char,
}

impl CXmpString {
    pub(crate) fn from_ptr(s: *const c_char) -> Self {
        Self { s }
    }

    pub(crate) fn as_string(&self) -> String {
        unsafe { CStr::from_ptr(self.s).to_string_lossy().into_owned() }
    }

    pub(crate) fn map<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce(String) -> U,
    {
        if self.s.is_null() {
            None
        } else {
            let s = self.as_string();
            Some(f(s))
        }
    }
}

impl Drop for CXmpString {
    fn drop(&mut self) {
        unsafe { CXmpStringDrop(self.s) };
    }
}

#[repr(C)]
pub(crate) struct CXmpError {
    pub(crate) had_error: u32,
    pub(crate) id: i32,
    pub(crate) debug_message: *const c_char,
}

impl CXmpError {
    #[allow(dead_code, clippy::unwrap_used)] // only used in test code
    pub(crate) fn new(had_error: bool, id: i32, debug_message: Option<&str>) -> Self {
        // Mimic a debug message coming from C++ code
        // so that we don't foul up our memory management
        // when this struct is dropped.

        Self {
            had_error: if had_error { 1 } else { 0 },
            id,
            debug_message: unsafe {
                match debug_message {
                    Some(debug_message) => {
                        let debug_message_as_cstr = CString::new(debug_message).unwrap();
                        CXmpStringCopy(debug_message_as_cstr.as_ptr())
                    }
                    None => std::ptr::null(),
                }
            },
        }
    }
}

impl Default for CXmpError {
    fn default() -> Self {
        Self {
            had_error: 0,
            id: 0,
            debug_message: std::ptr::null(),
        }
    }
}

impl Drop for CXmpError {
    fn drop(&mut self) {
        unsafe { CXmpStringDrop(self.debug_message) };
        self.debug_message = std::ptr::null();
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct CXmpDateTime {
    pub(crate) year: i32,
    pub(crate) month: i32,
    pub(crate) day: i32,
    pub(crate) hour: i32,
    pub(crate) minute: i32,
    pub(crate) second: i32,
    pub(crate) has_date: bool,
    pub(crate) has_time: bool,
    pub(crate) has_time_zone: bool,
    pub(crate) tz_sign: i8,
    pub(crate) tz_hour: i32,
    pub(crate) tz_minute: i32,
    pub(crate) nanosecond: i32,
}

pub(crate) enum CXmpFile {}
pub(crate) enum CXmpMeta {}

extern "C" {
    pub(crate) fn CXmpStringCopy(s: *const c_char) -> *const c_char;
    pub(crate) fn CXmpStringDrop(s: *const c_char);

    // --- CXmpFile ---

    pub(crate) fn CXmpFileNew(out_error: *mut CXmpError) -> *mut CXmpFile;
    pub(crate) fn CXmpFileDrop(file: *mut CXmpFile);

    pub(crate) fn CXmpFileOpen(
        file: *mut CXmpFile,
        out_error: *mut CXmpError,
        path: *const c_char,
        flags: u32,
    );

    pub(crate) fn CXmpFileClose(file: *mut CXmpFile);
    pub(crate) fn CXmpFileGetXmp(file: *mut CXmpFile) -> *mut CXmpMeta;

    pub(crate) fn CXmpFilePutXmp(
        file: *mut CXmpFile,
        out_error: *mut CXmpError,
        meta: *const CXmpMeta,
    );

    pub(crate) fn CXmpFileCanPutXmp(file: *const CXmpFile, meta: *const CXmpMeta) -> c_int;

    // --- CXmpMeta ---

    pub(crate) fn CXmpMetaNew(out_error: *mut CXmpError) -> *mut CXmpMeta;
    pub(crate) fn CXmpMetaDrop(meta: *mut CXmpMeta);

    pub(crate) fn CXmpMetaParseFromBuffer(
        out_error: *mut CXmpError,
        buffer: *const u8,
        buffer_size: u32,
    ) -> *mut CXmpMeta;

    pub(crate) fn CXmpMetaRegisterNamespace(
        out_error: *mut CXmpError,
        namespace_uri: *const c_char,
        suggested_prefix: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaGetProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_options: *mut u32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaGetProperty_Bool(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_value: *mut bool,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaGetProperty_Int(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_value: *mut i32,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaGetProperty_Int64(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_value: *mut i64,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaGetProperty_Float(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_value: *mut f64,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaGetProperty_Date(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_value: *mut CXmpDateTime,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaSetProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const c_char,
        options: u32,
    );

    pub(crate) fn CXmpMetaSetProperty_Bool(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: bool,
        options: u32,
    );

    pub(crate) fn CXmpMetaSetProperty_Int(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: i32,
        options: u32,
    );

    pub(crate) fn CXmpMetaSetProperty_Int64(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: i64,
        options: u32,
    );

    pub(crate) fn CXmpMetaSetProperty_Float(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: f64,
        options: u32,
    );

    pub(crate) fn CXmpMetaSetProperty_Date(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const CXmpDateTime,
        options: u32,
    );

    pub(crate) fn CXmpMetaDoesPropertyExist(
        meta: *const CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> c_int;

    pub(crate) fn CXmpMetaGetArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        index: u32,
        out_options: *mut u32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaGetLocalizedText(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        alt_text_name: *const c_char,
        generic_lang: *const c_char,
        specific_lang: *const c_char,
        out_actual_lang: *mut *const c_char,
        out_options: *mut u32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaComposeStructFieldPath(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
    ) -> *const c_char;

    // --- CXmpDateTime ---

    pub(crate) fn CXmpDateTimeCurrent(dt: *mut CXmpDateTime, out_error: *mut CXmpError);

    pub(crate) fn CXmpDateTimeToString(
        dt: *const CXmpDateTime,
        out_error: *mut CXmpError,
    ) -> *const c_char;
}
