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

/// The `XmpMeta` struct allows access to the XMP Toolkit core services.
///
/// You can create `XmpMeta` structs from metadata that you construct,
/// or that you obtain from files using the [`XmpFile`] struct.
///
/// `XmpMeta` implements `std::str::FromStr`, so you can create an `XmpMeta`
/// struct from a string that contains XMP as well.
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

    /// Gets a simple string property value.
    ///
    /// When specifying a namespace and path (in this and all other accessors):
    /// * If a namespace URI is specified, it must be for a registered
    ///   namespace.
    /// * If the namespace is specified only by a prefix in the property name
    ///   path, it must be a registered prefix.
    /// * If both a URI and path prefix are present, they must be corresponding
    ///   parts of a registered namespace.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI for the property. The URI must be for a
    ///   registered namespace. Must not be an empty string.
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. The first component can be a
    ///   namespace prefix; if present without a `schema_ns` value, the prefix
    ///   specifies the namespace. The prefix must be for a registered
    ///   namespace, and if a namespace URI is specified, must match the
    ///   registered prefix for that namespace.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn property(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<String>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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

    /// Gets a simple property value and interprets it as a bool.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a boolean (for example, it is
    /// an unrecognizable string), the function will return `None`.
    pub fn property_bool(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<bool>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`.
    pub fn property_i32(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<i32>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`.
    pub fn property_i64(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<i64>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a number, the function will
    /// return `None`. Note that ratio values, such as those found in
    /// TIFF and EXIF blocks, are not parsed.
    pub fn property_f64(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<f64>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    ///
    /// If the value can not be parsed as a date (for example, it is
    /// an unrecognizable string), the function will return `None`.
    pub fn property_date(&self, schema_ns: &str, prop_name: &str) -> Option<XmpValue<XmpDateTime>> {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let c_value = CString::new(prop_value.value.as_bytes())?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                c_value.as_ptr(),
                prop_value.options,
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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_bool(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<bool>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Bool(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                prop_value.value,
                prop_value.options,
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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_i32(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<i32>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Int(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                prop_value.value,
                prop_value.options,
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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_i64(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<i64>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Int64(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                prop_value.value,
                prop_value.options,
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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_f64(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<f64>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Float(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                prop_value.value,
                prop_value.options,
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
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_date(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpValue<XmpDateTime>,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns)?;
        let c_name = CString::new(prop_name)?;
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty_Date(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                &prop_value.value.as_ffi(),
                prop_value.options,
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates an iterator for an array property value.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    pub fn array_property(&self, schema_ns: &str, prop_name: &str) -> ArrayProperty {
        ArrayProperty {
            meta: self,
            ns: CString::new(schema_ns).unwrap_or_default(),
            name: CString::new(prop_name).unwrap_or_default(),
            index: 1,
        }
    }

    /// Reports whether a property currently exists.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general path
    ///   expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `false` in such cases.
    pub fn does_property_exist(&self, schema_ns: &str, prop_name: &str) -> bool {
        let c_ns = CString::new(schema_ns).unwrap_or_default();
        let c_name = CString::new(prop_name).unwrap_or_default();

        let r = unsafe { ffi::CXmpMetaDoesPropertyExist(self.m, c_ns.as_ptr(), c_name.as_ptr()) };
        r != 0
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
/// Create via [`XmpMeta::array_property`].
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
