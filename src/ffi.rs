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

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub(crate) struct CXmpError {
    pub(crate) had_error: u32,
    pub(crate) id: i32,
    pub(crate) debug_message: *const c_char,
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
    ) -> *mut c_char;

    pub(crate) fn CXmpMetaGetProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        out_options: *mut u32,
    ) -> *mut c_char;

    pub(crate) fn CXmpMetaSetProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const c_char,
        options: u32,
    );

    pub(crate) fn CXmpMetaDoesPropertyExist(
        meta: *const CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> c_int;

    pub(crate) fn CXmpMetaSetPropertyDate(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const CXmpDateTime,
        options: u32,
    );

    pub(crate) fn CXmpMetaGetArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        index: u32,
        out_options: *mut u32,
    ) -> *mut c_char;

    // --- CXmpDateTime ---

    pub(crate) fn CXmpDateTimeCurrent(dt: *mut CXmpDateTime, out_error: *mut CXmpError);
}
