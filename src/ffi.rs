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

pub(crate) enum CXmpDateTime {}
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
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> *mut c_char;

    pub(crate) fn CXmpMetaSetProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const c_char,
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
    );

    // --- CXmpDateTime ---

    pub(crate) fn CXmpDateTimeNew() -> *mut CXmpDateTime;
    pub(crate) fn CXmpDateTimeDrop(dt: *mut CXmpDateTime);
    pub(crate) fn CXmpDateTimeCurrent(out_error: *mut CXmpError) -> *mut CXmpDateTime;
}
