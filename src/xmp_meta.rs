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
    path::Path,
    str::FromStr,
};

use crate::{
    ffi, OpenFileOptions, XmpDateTime, XmpError, XmpErrorType, XmpFile, XmpResult, XmpValue,
};

/// An `XmpMeta` struct allows you to inspect and modify the data model
/// of an XMP packet.
///
/// You can create an `XmpMeta` struct by:
/// * Creating an empty struct ([`XmpMeta::new`])
/// * Reading metadata from a file ([`XmpFile::xmp`])
/// * Parsing a string containing metadata ([`XmpMeta::from_str`])
///
/// ## Accessing properties
///
/// Many of the methods on this struct allow you to access or modify a
/// **property.** Every property in XMP is identified using two arguments:
///
/// * **`namespace`** may be either a URI or a prefix. If a URI is used, it must
///   have been registered via ([`XmpMeta::register_namespace`]) or be built-in
///   to the XMP Toolkit (see [`xmp_ns`](crate::xmp_ns) for constants you may
///   use in this way). If a prefix is used, it must be a prefix returned after
///   having called [`XmpMeta::register_namespace`]. If both a URI and path
///   prefix are present, they must be corresponding parts of a registered
///   namespace.
/// * **`path`** specifies a path to the property. In the simplest case, this is
///   a simple string identifier within `namespace`, but it can also be a path
///   expression. Must not be an empty string. The first component of a path
///   expression can be a namespace prefix; if so, the prefix must have been
///   registered via [`XmpMeta::register_namespace`].
pub struct XmpMeta {
    pub(crate) m: *mut ffi::CXmpMeta,
}

impl Drop for XmpMeta {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpMetaDrop(self.m);
        }
    }
}

impl XmpMeta {
    /// Creates a new, empty metadata struct.
    ///
    /// An error result from this function is unlikely but possible
    /// if, for example, the C++ XMP Toolkit fails to initialize or
    /// reports an out-of-memory condition.
    pub fn new() -> XmpResult<XmpMeta> {
        let mut err = ffi::CXmpError::default();
        let m = unsafe { ffi::CXmpMetaNew(&mut err) };
        XmpError::raise_from_c(&err)?;

        Ok(XmpMeta { m })
    }

    /// Reads the XMP from a file without keeping the file open.
    ///
    /// This is a convenience function for read-only workflows.
    ///
    /// ## Arguments
    ///
    /// * `path`: Path to the file to be read
    pub fn from_file<P: AsRef<Path>>(path: P) -> XmpResult<Self> {
        let mut f = XmpFile::new()?;
        f.open_file(path, OpenFileOptions::default().only_xmp())?;

        f.xmp().ok_or_else(|| XmpError {
            error_type: XmpErrorType::Unavailable,
            debug_message: "No XMP in file".to_owned(),
        })
    }

    /// Registers a namespace URI with a suggested prefix.
    ///
    /// If the URI is not registered but the suggested prefix
    /// is in use, a unique prefix is created from the suggested one.
    /// The actual registered prefix is returned. It is not an error
    /// if the URI is already registered, regardless of the prefix.
    ///
    /// ## Arguments
    ///
    /// * `namespace_uri`: The URI for the namespace. Must be a valid XML URI.
    ///
    /// * `suggested_prefix`: The suggested prefix to be used if the URI is not
    ///   yet registered. Must be a valid XML name.
    ///
    /// Returns the prefix actually registered for this URI.
    pub fn register_namespace(namespace_uri: &str, suggested_prefix: &str) -> XmpResult<String> {
        // These .unwrap() calls are deemed unlikely to panic as this
        // function is typically called with known, standardized strings
        // in the ASCII space.
        let c_ns = CString::new(namespace_uri).unwrap_or_default();
        let c_sp = CString::new(suggested_prefix).unwrap_or_default();

        unsafe {
            let mut err = ffi::CXmpError::default();

            let c_result = ffi::CXmpMetaRegisterNamespace(&mut err, c_ns.as_ptr(), c_sp.as_ptr());

            XmpError::raise_from_c(&err)?;

            Ok(CStr::from_ptr(c_result).to_string_lossy().into_owned())
        }
    }

    /// Returns `true` if the metadata block contains a property by this name.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `false` in such cases.
    pub fn contains_property(&self, namespace: &str, path: &str) -> bool {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let r = unsafe { ffi::CXmpMetaDoesPropertyExist(self.m, c_ns.as_ptr(), c_name.as_ptr()) };
        r != 0
    }

    /// Gets a simple string property value.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn property(&self, namespace: &str, path: &str) -> Option<XmpValue<String>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut err = ffi::CXmpError::default();

        unsafe {
            let c_result = ffi::CXmpMetaGetProperty(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut options,
            );

            if c_result.is_null() {
                None
            } else {
                Some(XmpValue {
                    value: CStr::from_ptr(c_result).to_string_lossy().into_owned(),
                    options,
                })
            }
        }
    }

    /// Creates an iterator for an array property value.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    pub fn property_array(&self, namespace: &str, path: &str) -> ArrayProperty {
        ArrayProperty {
            meta: self,
            ns: CString::new(namespace).unwrap_or_default(),
            name: CString::new(path).unwrap_or_default(),
            index: 1,
        }
    }

    /// Gets a simple property value and interprets it as a bool.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a boolean (for example, it is
    /// an unrecognizable string), the function will return `None`.
    pub fn property_bool(&self, namespace: &str, path: &str) -> Option<XmpValue<bool>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut value = false;
        let mut err = ffi::CXmpError::default();

        unsafe {
            if ffi::CXmpMetaGetProperty_Bool(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut value,
                &mut options,
            ) {
                Some(XmpValue { value, options })
            } else {
                None
            }
        }
    }

    /// Gets a simple property value and interprets it as a 32-bit integer.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`.
    pub fn property_i32(&self, namespace: &str, path: &str) -> Option<XmpValue<i32>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut value: i32 = 0;
        let mut err = ffi::CXmpError::default();

        unsafe {
            if ffi::CXmpMetaGetProperty_Int(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut value,
                &mut options,
            ) {
                Some(XmpValue { value, options })
            } else {
                None
            }
        }
    }

    /// Gets a simple property value and interprets it as a 64-bit integer.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`.
    pub fn property_i64(&self, namespace: &str, path: &str) -> Option<XmpValue<i64>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut value: i64 = 0;
        let mut err = ffi::CXmpError::default();

        unsafe {
            if ffi::CXmpMetaGetProperty_Int64(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut value,
                &mut options,
            ) {
                Some(XmpValue { value, options })
            } else {
                None
            }
        }
    }

    /// Gets a simple property value and interprets it as a 64-bit float.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`. Note that ratio values, such as those found in
    /// TIFF and EXIF blocks, are not parsed.
    pub fn property_f64(&self, namespace: &str, path: &str) -> Option<XmpValue<f64>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut value: f64 = 0.0;
        let mut err = ffi::CXmpError::default();

        unsafe {
            if ffi::CXmpMetaGetProperty_Float(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut value,
                &mut options,
            ) {
                Some(XmpValue { value, options })
            } else {
                None
            }
        }
    }

    /// Gets a simple property value and interprets it as a date/time value.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a date (for example, it is
    /// an unrecognizable string), the function will return `None`.
    pub fn property_date(&self, namespace: &str, path: &str) -> Option<XmpValue<XmpDateTime>> {
        let c_ns = CString::new(namespace).unwrap_or_default();
        let c_name = CString::new(path).unwrap_or_default();

        let mut options: u32 = 0;
        let mut value = ffi::CXmpDateTime::default();
        let mut err = ffi::CXmpError::default();

        unsafe {
            if ffi::CXmpMetaGetProperty_Date(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &mut value,
                &mut options,
            ) {
                Some(XmpValue {
                    value: XmpDateTime::from_ffi(&value),
                    options,
                })
            } else {
                None
            }
        }
    }

    /// Creates or sets a property value.
    ///
    /// This is the simplest property setter. Use it for top-level
    /// simple properties.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let c_value = CString::new(new_value.value.as_bytes())?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                c_value.as_ptr(),
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using a bool value.
    ///
    /// Since XMP only stores strings, the bool value will be converted to
    /// a string (`"True"` or `"False"`) as part of this operation.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property_bool(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<bool>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Bool(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                new_value.value,
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using a 32-bit integer value.
    ///
    /// Since XMP only stores strings, the integer value will be converted to
    /// a string as part of this operation.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property_i32(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<i32>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Int(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                new_value.value,
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using a 64-bit integer value.
    ///
    /// Since XMP only stores strings, the integer value will be converted to
    /// a string as part of this operation.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property_i64(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<i64>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Int64(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                new_value.value,
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using a 64-bit floating-point value.
    ///
    /// Since XMP only stores strings, the float value will be converted to
    /// a string as part of this operation.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property_f64(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<f64>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Float(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                new_value.value,
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using an [`XmpDateTime`] structure.
    ///
    /// Since XMP only stores strings, the date/time will be converted to
    /// ISO 8601 format as part of this operation.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `new_value`: The new value.
    pub fn set_property_date(
        &mut self,
        namespace: &str,
        path: &str,
        new_value: &XmpValue<XmpDateTime>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(namespace)?;
        let c_name = CString::new(path)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Date(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &new_value.value.as_ffi(),
                new_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }
}

impl FromStr for XmpMeta {
    type Err = XmpError;

    /// Creates a new `XmpMeta` struct and populates it with metadata from a
    /// string containing serialized RDF. This string must be a complete RDF
    /// parse stream.
    ///
    /// ## Arguments
    ///
    /// * `xmp`: XMP string to be read
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut err = ffi::CXmpError::default();
        let bytes = s.as_bytes();
        let m =
            unsafe { ffi::CXmpMetaParseFromBuffer(&mut err, bytes.as_ptr(), bytes.len() as u32) };
        XmpError::raise_from_c(&err)?;

        Ok(XmpMeta { m })
    }
}

/// An iterator that provides access to items within a property array.
///
/// Create via [`XmpMeta::property_array`].
pub struct ArrayProperty<'a> {
    meta: &'a XmpMeta,
    ns: CString,
    name: CString,
    index: u32,
}

impl<'a> Iterator for ArrayProperty<'a> {
    type Item = XmpValue<String>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            let c_result = ffi::CXmpMetaGetArrayItem(
                self.meta.m,
                &mut err,
                self.ns.as_ptr(),
                self.name.as_ptr(),
                self.index,
                &mut options,
            );

            self.index += 1;

            if c_result.is_null() {
                None
            } else {
                Some(XmpValue {
                    value: CStr::from_ptr(c_result).to_string_lossy().into_owned(),
                    options,
                })
            }
        }
    }
}
