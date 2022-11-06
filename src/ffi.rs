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
    os::raw::{c_char, c_int, c_void},
    slice,
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

type CXmpTextOutputProc = extern "C" fn(s: *mut c_void, buffer: *const u8, len: u32) -> i32;

// Implementation of CXmpTextOutputProc that appends buffer to a Rust String.
pub(crate) extern "C" fn xmp_dump_to_string(s: *mut c_void, buffer: *const u8, len: u32) -> i32 {
    unsafe {
        let cstr = slice::from_raw_parts(buffer, len as usize);
        let cstr = String::from_utf8_lossy(cstr);
        let s = &mut *s.cast::<String>();
        s.push_str(cstr.as_ref());
    }

    0
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
            had_error: u32::from(had_error),
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
pub(crate) enum CXmpIterator {}

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

    pub(crate) fn CXmpMetaClone(m: *const CXmpMeta, out_error: *mut CXmpError) -> *mut CXmpMeta;

    pub(crate) fn CXmpMetaParseFromBuffer(
        out_error: *mut CXmpError,
        buffer: *const u8,
        buffer_size: u32,
        options: u32,
    ) -> *mut CXmpMeta;

    pub(crate) fn CXmpMetaSerializeToBuffer(
        meta: *const CXmpMeta,
        out_error: *mut CXmpError,
        options: u32,
        padding: u32,
        newline: *const c_char,
        indent: *const c_char,
        base_indent: u32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaRegisterNamespace(
        out_error: *mut CXmpError,
        namespace_uri: *const c_char,
        suggested_prefix: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaGetNamespacePrefix(
        out_error: *mut CXmpError,
        namespace_uri: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaGetNamespaceURI(
        out_error: *mut CXmpError,
        namespace_prefix: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpDumpNamespaces(out_string: *mut c_void, callback: CXmpTextOutputProc);

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

    pub(crate) fn CXmpMetaGetStructField(
        meta: *const CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
        out_options: *mut u32,
    ) -> *const c_char;

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

    pub(crate) fn CXmpMetaDeleteProperty(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    );

    pub(crate) fn CXmpMetaSetArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        array_name: *const c_char,
        item_index: u32,
        item_value: *const c_char,
        item_options: u32,
    );

    pub(crate) fn CXmpMetaAppendArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        array_name: *const c_char,
        array_options: u32,
        item_value: *const c_char,
        item_options: u32,
    );

    pub(crate) fn CXmpMetaDeleteArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        array_name: *const c_char,
        item_index: i32,
    );

    pub(crate) fn CXmpMetaCountArrayItems(
        meta: *const CXmpMeta,
        out_error: *mut CXmpError,
        array_ns: *const c_char,
        array_name: *const c_char,
        count: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpMetaSetStructField(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
        item_value: *const c_char,
        item_options: u32,
    );

    pub(crate) fn CXmpMetaDeleteStructField(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
    );

    pub(crate) fn CXmpMetaGetQualifier(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        qual_ns: *const c_char,
        qual_name: *const c_char,
        out_qual_options: *mut u32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaSetQualifier(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        qual_ns: *const c_char,
        qual_name: *const c_char,
        qual_value: *const c_char,
        qual_options: u32,
    );

    pub(crate) fn CXmpMetaDeleteQualifier(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        prop_ns: *const c_char,
        prop_name: *const c_char,
        qual_ns: *const c_char,
        qual_name: *const c_char,
    );

    pub(crate) fn CXmpMetaDoesPropertyExist(
        meta: *const CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> c_int;

    pub(crate) fn CXmpMetaDoesStructFieldExist(
        meta: *const CXmpMeta,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
    ) -> c_int;

    pub(crate) fn CXmpMetaDoesQualifierExist(
        meta: *const CXmpMeta,
        prop_ns: *const c_char,
        prop_name: *const c_char,
        qual_ns: *const c_char,
        qual_name: *const c_char,
    ) -> c_int;

    pub(crate) fn CXmpMetaGetArrayItem(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        index: i32,
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

    pub(crate) fn CXmpMetaSetLocalizedText(
        meta: *const CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        alt_text_name: *const c_char,
        generic_lang: *const c_char,
        specific_lang: *const c_char,
        item_value: *const c_char,
        options: u32,
    );

    pub(crate) fn CXmpMetaSort(meta: *mut CXmpMeta, out_error: *mut CXmpError);

    pub(crate) fn CXmpMetaGetObjectName(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaSetObjectName(
        meta: *mut CXmpMeta,
        out_error: *mut CXmpError,
        name: *const c_char,
    );

    pub(crate) fn CXmpMetaComposeArrayItemPath(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        array_name: *const c_char,
        index: i32,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaComposeLangSelector(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        array_name: *const c_char,
        lang_name: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaComposeFieldSelector(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
        field_value: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaComposeQualifierPath(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        qual_ns: *const c_char,
        qual_name: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaComposeStructFieldPath(
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        struct_name: *const c_char,
        field_ns: *const c_char,
        field_name: *const c_char,
    ) -> *const c_char;

    pub(crate) fn CXmpMetaDumpObj(
        meta: *mut CXmpMeta,
        out_string: *mut c_void,
        callback: CXmpTextOutputProc,
    );

    // --- CXmpIterator ---

    pub(crate) fn CXmpIteratorNew(
        m: *const CXmpMeta,
        out_error: *mut CXmpError,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        options: u32,
    ) -> *mut CXmpIterator;

    pub(crate) fn CXmpIteratorDrop(i: *mut CXmpIterator);

    pub(crate) fn CXmpIteratorNext(
        i: *mut CXmpIterator,
        out_error: *mut CXmpError,
        out_schema_ns: *mut *const c_char,
        out_prop_path: *mut *const c_char,
        out_prop_value: *mut *const c_char,
        out_options: *mut u32,
    ) -> bool;

    pub(crate) fn CXmpIteratorSkip(i: *mut CXmpIterator, out_error: *mut CXmpError, options: u32);

    // --- CXmpDateTime ---

    pub(crate) fn CXmpDateTimeCurrent(dt: *mut CXmpDateTime, out_error: *mut CXmpError);

    pub(crate) fn CXmpDateTimeSetTimeZone(dt: *mut CXmpDateTime, out_error: *mut CXmpError);
    pub(crate) fn CXmpDateTimeConvertToLocalTime(dt: *mut CXmpDateTime, out_error: *mut CXmpError);
    pub(crate) fn CXmpDateTimeConvertToUTCTime(dt: *mut CXmpDateTime, out_error: *mut CXmpError);

    pub(crate) fn CXmpDateTimeToString(
        dt: *const CXmpDateTime,
        out_error: *mut CXmpError,
    ) -> *const c_char;
}
