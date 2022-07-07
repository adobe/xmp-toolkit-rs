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

pub(crate) enum CXmpDateTime {}
pub(crate) enum CXmpFile {}
pub(crate) enum CXmpMeta {}

extern "C" {
    // --- CXmpFile ---

    pub(crate) fn CXmpFileNew() -> *mut CXmpFile;
    pub(crate) fn CXmpFileDrop(file: *mut CXmpFile);
    pub(crate) fn CXmpFileOpen(file: *mut CXmpFile, path: *const c_char, flags: u32) -> c_int;
    pub(crate) fn CXmpFileClose(file: *mut CXmpFile);
    pub(crate) fn CXmpFileGetXmp(file: *mut CXmpFile) -> *mut CXmpMeta;
    pub(crate) fn CXmpFilePutXmp(file: *mut CXmpFile, meta: *const CXmpMeta);
    pub(crate) fn CXmpFileCanPutXmp(file: *const CXmpFile, meta: *const CXmpMeta) -> c_int;

    // --- CXmpMeta ---

    pub(crate) fn CXmpMetaNew() -> *mut CXmpMeta;
    pub(crate) fn CXmpMetaDrop(meta: *mut CXmpMeta);

    pub(crate) fn CXmpMetaRegisterNamespace(
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
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const CXmpDateTime,
    );

    // --- CXmpDateTime ---

    pub(crate) fn CXmpDateTimeNew() -> *mut CXmpDateTime;
    pub(crate) fn CXmpDateTimeDrop(dt: *mut CXmpDateTime);
    pub(crate) fn CXmpDateTimeCurrent() -> *mut CXmpDateTime;
}
