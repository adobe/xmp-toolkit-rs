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
    ffi::CString,
    fmt,
    os::raw::{c_char, c_void},
    path::Path,
    str::FromStr,
};

use crate::{
    ffi::{self, CXmpString},
    IterOptions, OpenFileOptions, XmpDateTime, XmpError, XmpErrorType, XmpFile, XmpIterator,
    XmpResult, XmpValue,
};

/// Represents the data model of an XMP packet.
///
/// You can create an `XmpMeta` struct by:
/// * Creating an empty struct ([`XmpMeta::new`]).
/// * Reading metadata from a file ([`XmpFile::xmp`]).
/// * Parsing a string containing metadata ([`XmpMeta::from_str`]).
///
/// There are many methods on this struct which allow you to inspect
/// and modify the XMP data model.
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
    pub(crate) m: Option<*mut ffi::CXmpMeta>,
}

impl Drop for XmpMeta {
    fn drop(&mut self) {
        if let Some(m) = self.m {
            unsafe {
                ffi::CXmpMetaDrop(m);
            }
        }
    }
}

impl XmpMeta {
    /// Special index value which references the last item in an array,
    /// regardless of the array index.
    pub const LAST_ITEM: i32 = -1;

    /// Creates a new, empty metadata struct.
    ///
    /// An error result from this function is unlikely but possible
    /// if, for example, the C++ XMP Toolkit fails to initialize or
    /// reports an out-of-memory condition.
    pub fn new() -> XmpResult<Self> {
        let mut err = ffi::CXmpError::default();
        let m = unsafe { ffi::CXmpMetaNew(&mut err) };
        XmpError::raise_from_c(&err)?;

        Ok(Self { m: Some(m) })
    }

    /// Use only for testing. Simulates failure to initialize
    /// C++ XMP Toolkit.
    #[allow(dead_code)] // used only in test code
    pub(crate) fn new_fail() -> Self {
        Self { m: None }
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
    /// **IMPORTANT:** Namespace registrations are global state in
    /// the C++ XMP Toolkit and not related to any single data model.
    /// For this reason, the corresponding function (i.e. to unregister
    /// a namespace) is not provided.
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
        let c_ns = CString::new(namespace_uri).unwrap_or_default();
        let c_sp = CString::new(suggested_prefix).unwrap_or_default();

        unsafe {
            let mut err = ffi::CXmpError::default();

            let result = CXmpString::from_ptr(ffi::CXmpMetaRegisterNamespace(
                &mut err,
                c_ns.as_ptr(),
                c_sp.as_ptr(),
            ));

            XmpError::raise_from_c(&err)?;

            Ok(result.as_string())
        }
    }

    /// Returns the prefix for a registered namespace URI if it exists.
    ///
    /// **IMPORTANT:** Namespace registrations are global state in
    /// the C++ XMP Toolkit and not related to any single data model.
    ///
    /// ## Arguments
    ///
    /// * `namespace_uri`: The URI for the namespace. Must be a valid XML URI.
    pub fn namespace_prefix(namespace_uri: &str) -> Option<String> {
        let c_ns = CString::new(namespace_uri).unwrap_or_default();

        unsafe {
            let mut err = ffi::CXmpError::default();

            let result =
                CXmpString::from_ptr(ffi::CXmpMetaGetNamespacePrefix(&mut err, c_ns.as_ptr()));

            result.map(|s| s)
        }
    }

    /// Returns the URL for a registered namespace prefix if it exists.
    ///
    /// **IMPORTANT:** Namespace registrations are global state in
    /// the C++ XMP Toolkit and not related to any single data model.
    ///
    /// ## Arguments
    ///
    /// * `namespace_prefix`: The prefix for the namespace.
    pub fn namespace_uri(namespace_prefix: &str) -> Option<String> {
        let c_prefix = CString::new(namespace_prefix).unwrap_or_default();

        unsafe {
            let mut err = ffi::CXmpError::default();

            let result =
                CXmpString::from_ptr(ffi::CXmpMetaGetNamespaceURI(&mut err, c_prefix.as_ptr()));

            result.map(|s| s)
        }
    }

    /// Returns a list of registered namespaces as a string.
    ///
    /// Intended for debugging/logging use.
    ///
    /// **IMPORTANT:** Namespace registrations are global state in
    /// the C++ XMP Toolkit and not related to any single data model.
    pub fn debug_dump_namespaces() -> String {
        let mut result = String::default();

        unsafe {
            let result: *mut String = &mut result;
            ffi::CXmpDumpNamespaces(
                std::mem::transmute::<*mut String, *mut c_void>(result),
                ffi::xmp_dump_to_string,
            );
        }

        result
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let r = unsafe { ffi::CXmpMetaDoesPropertyExist(m, c_ns.as_ptr(), c_name.as_ptr()) };
            r != 0
        } else {
            false
        }
    }

    /// Returns `true` if the metadata block contains a struct field by this
    /// name.
    ///
    /// ## Arguments
    ///
    /// * `struct_ns` and `struct_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `false` in such cases.
    pub fn contains_struct_field(
        &self,
        struct_ns: &str,
        struct_path: &str,
        field_ns: &str,
        field_name: &str,
    ) -> bool {
        if let Some(m) = self.m {
            let c_struct_ns = CString::new(struct_ns).unwrap_or_default();
            let c_struct_name = CString::new(struct_path).unwrap_or_default();
            let c_field_ns = CString::new(field_ns).unwrap_or_default();
            let c_field_name = CString::new(field_name).unwrap_or_default();

            unsafe {
                ffi::CXmpMetaDoesStructFieldExist(
                    m,
                    c_struct_ns.as_ptr(),
                    c_struct_name.as_ptr(),
                    c_field_ns.as_ptr(),
                    c_field_name.as_ptr(),
                ) != 0
            }
        } else {
            false
        }
    }

    /// Returns `true` if the metadata block contains the named qualifier.
    ///
    /// ## Arguments
    ///
    /// * `prop_ns` and `prop_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `qual` and `qual_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `false` in such cases.
    pub fn contains_qualifier(
        &self,
        prop_ns: &str,
        prop_path: &str,
        qual_ns: &str,
        qual_name: &str,
    ) -> bool {
        if let Some(m) = self.m {
            let c_prop_ns = CString::new(prop_ns).unwrap_or_default();
            let c_prop_name = CString::new(prop_path).unwrap_or_default();
            let c_qual_ns = CString::new(qual_ns).unwrap_or_default();
            let c_qual_name = CString::new(qual_name).unwrap_or_default();

            unsafe {
                ffi::CXmpMetaDoesQualifierExist(
                    m,
                    c_prop_ns.as_ptr(),
                    c_prop_name.as_ptr(),
                    c_qual_ns.as_ptr(),
                    c_qual_name.as_ptr(),
                ) != 0
            }
        } else {
            false
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                CXmpString::from_ptr(ffi::CXmpMetaGetProperty(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    &mut options,
                ))
                .map(|value| XmpValue { value, options })
            }
        } else {
            None
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
            index: 0,
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut value = false;
            let mut err = ffi::CXmpError::default();

            unsafe {
                if ffi::CXmpMetaGetProperty_Bool(
                    m,
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
        } else {
            None
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut value: i32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                if ffi::CXmpMetaGetProperty_Int(
                    m,
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
        } else {
            None
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut value: i64 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                if ffi::CXmpMetaGetProperty_Int64(
                    m,
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
        } else {
            None
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
    /// TIFF and Exif blocks, are not parsed.
    pub fn property_f64(&self, namespace: &str, path: &str) -> Option<XmpValue<f64>> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut value: f64 = 0.0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                if ffi::CXmpMetaGetProperty_Float(
                    m,
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
        } else {
            None
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();

            let mut options: u32 = 0;
            let mut value = ffi::CXmpDateTime::default();
            let mut err = ffi::CXmpError::default();

            unsafe {
                if ffi::CXmpMetaGetProperty_Date(
                    m,
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
        } else {
            None
        }
    }

    /// Gets a field value from within an nested structure.
    ///
    /// ## Arguments
    ///
    /// * `struct_ns` and `struct_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn struct_field(
        &self,
        struct_ns: &str,
        struct_path: &str,
        field_ns: &str,
        field_name: &str,
    ) -> Option<XmpValue<String>> {
        if let Some(m) = self.m {
            let c_struct_ns = CString::new(struct_ns).unwrap_or_default();
            let c_struct_name = CString::new(struct_path).unwrap_or_default();
            let c_field_ns = CString::new(field_ns).unwrap_or_default();
            let c_field_name = CString::new(field_name).unwrap_or_default();

            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                CXmpString::from_ptr(ffi::CXmpMetaGetStructField(
                    m,
                    &mut err,
                    c_struct_ns.as_ptr(),
                    c_struct_name.as_ptr(),
                    c_field_ns.as_ptr(),
                    c_field_name.as_ptr(),
                    &mut options,
                ))
                .map(|value| XmpValue { value, options })
            }
        } else {
            None
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let c_value = CString::new(new_value.value.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    if new_value.value.is_empty() {
                        std::ptr::null()
                    } else {
                        c_value.as_ptr()
                    },
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty_Bool(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    new_value.value,
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty_Int(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    new_value.value,
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty_Int64(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    new_value.value,
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty_Float(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    new_value.value,
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
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
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetProperty_Date(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    &new_value.value.as_ffi(),
                    new_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Deletes an XMP subtree rooted at a given property.
    ///
    /// It is not an error if the qualifier does not exist.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    pub fn delete_property(&mut self, namespace: &str, path: &str) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_name = CString::new(path)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaDeleteProperty(m, &mut err, c_ns.as_ptr(), c_name.as_ptr());
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Provides access to items within an array.
    ///
    /// Use `XmpMeta::compose_array_item_path` to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `array_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `item_index`: Index into the array. **IMPORTANT:** Indices in XMP are
    ///   1-based, unlike Rust where indices are typically 0-based.  Use
    ///   [`XmpMeta::LAST_ITEM`] to specify the last existing array item.
    pub fn array_item(
        &self,
        namespace: &str,
        array_name: &str,
        item_index: i32,
    ) -> Option<XmpValue<String>> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_array_name = CString::new(array_name).unwrap_or_default();

            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                CXmpString::from_ptr(ffi::CXmpMetaGetArrayItem(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_array_name.as_ptr(),
                    item_index,
                    &mut options,
                ))
                .map(|value| XmpValue { value, options })
            }
        } else {
            None
        }
    }

    /// Creates or sets the value of an item within an array.
    ///
    /// Items are accessed by an integer index, where the first item has index
    /// 1. This function creates the item if necessary, but the array itself
    /// must already exist. Use [`XmpMeta::append_array_item`] to create
    /// arrays. A new item is automatically appended if the index is the array
    /// size plus 1.
    ///
    /// Use `XmpMeta::compose_array_item_path` to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `array_name`: See [Accessing
    ///   properties](#accessing-properties). NOTE: `array_name` is an
    ///   `XmpValue<String>` which contains any necessary flags for the array.
    /// * `item_placement`: Describes where to place the new item. See
    ///   [`ItemPlacement`].
    /// * `item_value`: Contains value and flags for the item to be added to the
    ///   array.
    pub fn set_array_item(
        &mut self,
        namespace: &str,
        array_name: &str,
        item_placement: ItemPlacement,
        item_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_array_name = CString::new(array_name)?;
            let c_item_value = CString::new(item_value.value.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            let mut options = item_value.options;
            let item_index = match item_placement {
                ItemPlacement::InsertAfterIndex(index) => {
                    options |= 0x8000;
                    index
                }
                ItemPlacement::InsertBeforeIndex(index) => {
                    options |= 0x4000;
                    index
                }
                ItemPlacement::ReplaceItemAtIndex(index) => index,
            };

            unsafe {
                ffi::CXmpMetaSetArrayItem(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_array_name.as_ptr(),
                    item_index,
                    c_item_value.as_ptr(),
                    options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Adds an item to an array, creating the array if necessary.
    ///
    /// This function simplifies construction of an array by not requiring
    /// that you pre-create an empty array. The array that is assigned is
    /// created automatically if it does not yet exist. If the array exists,
    /// it must have the form specified by the flags on `array_name`.
    ///
    /// Each call appends a new item to the array.
    ///
    /// Use [`XmpMeta::compose_array_item_path`] to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `array_name`: See [Accessing
    ///   properties](#accessing-properties). NOTE: `array_name` is an
    ///   `XmpValue<String>` which contains any necessary flags for the array.
    /// * `item_value`: Contains value and flags for the item to be added to the
    ///   array.
    pub fn append_array_item(
        &mut self,
        namespace: &str,
        array_name: &XmpValue<String>,
        item_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_array_name = CString::new(array_name.value.as_bytes())?;
            let c_item_value = CString::new(item_value.value.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaAppendArrayItem(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_array_name.as_ptr(),
                    array_name.options,
                    c_item_value.as_ptr(),
                    item_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Deletes an XMP subtree rooted at a given array item.
    ///
    /// It is not an error if the array item does not exist. Use
    /// [`XmpMeta::compose_array_item_path`] to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `array_name`: See [Accessing
    ///   properties](#accessing-properties). NOTE: `array_name` is an
    ///   `XmpValue<String>` which contains any necessary flags for the array.
    /// * `item_index`: The index of the desired item. Use
    ///   [`XmpMeta::LAST_ITEM`] to specify the last existing array item.
    ///   **IMPORTANT:** Indices in XMP are 1-based, not zero-based as in most
    ///   of Rust.
    pub fn delete_array_item(
        &mut self,
        namespace: &str,
        array_name: &str,
        item_index: i32,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace)?;
            let c_array_name = CString::new(array_name)?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaDeleteArrayItem(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_array_name.as_ptr(),
                    item_index,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Reports the number of items currently defined in an array.
    ///
    /// ## Arguments
    ///
    /// * `array_ns` and `array_name`: See [Accessing
    ///   properties](#accessing-properties).
    ///
    /// If any error occurs (for instance, the array does not exist),
    /// this function will return 0.
    pub fn array_len(&self, array_ns: &str, array_name: &str) -> usize {
        let mut result: u32 = 0;

        if let Some(m) = self.m {
            let c_array_ns = CString::new(array_ns).unwrap_or_default();
            let c_array_name = CString::new(array_name.as_bytes()).unwrap_or_default();
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaCountArrayItems(
                    m,
                    &mut err,
                    c_array_ns.as_ptr(),
                    c_array_name.as_ptr(),
                    &mut result,
                );
            }
        }

        result as usize
    }

    /// Creates or sets the value of a field within a nested structure,
    /// using a string value.
    ///
    /// Use this function to set a value within an existing structure,
    /// create a new field within an existing structure, or create an
    /// empty structure of any depth. If you set a field in a structure
    /// that does not exist, the structure is automatically created.
    ///
    /// Use [`XmpMeta::compose_struct_field_path`] to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `struct_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    /// * `item_value`: Contains value and flags for the item to be added to the
    ///   struct.
    pub fn set_struct_field(
        &mut self,
        namespace: &str,
        struct_name: &str,
        field_ns: &str,
        field_name: &str,
        item_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_struct_ns = CString::new(namespace)?;
            let c_struct_name = CString::new(struct_name.as_bytes())?;
            let c_field_ns = CString::new(field_ns)?;
            let c_field_name = CString::new(field_name.as_bytes())?;
            let c_item_value = CString::new(item_value.value.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetStructField(
                    m,
                    &mut err,
                    c_struct_ns.as_ptr(),
                    c_struct_name.as_ptr(),
                    c_field_ns.as_ptr(),
                    c_field_name.as_ptr(),
                    c_item_value.as_ptr(),
                    item_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Deletes an XMP subtree rooted at a given struct field.
    ///
    /// It is not an error if the field does not exist.
    ///
    /// Use [`XmpMeta::compose_struct_field_path`] to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `struct_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    pub fn delete_struct_field(
        &mut self,
        namespace: &str,
        struct_name: &str,
        field_ns: &str,
        field_name: &str,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_struct_ns = CString::new(namespace)?;
            let c_struct_name = CString::new(struct_name.as_bytes())?;
            let c_field_ns = CString::new(field_ns)?;
            let c_field_name = CString::new(field_name.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaDeleteStructField(
                    m,
                    &mut err,
                    c_struct_ns.as_ptr(),
                    c_struct_name.as_ptr(),
                    c_field_ns.as_ptr(),
                    c_field_name.as_ptr(),
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Provides access to a qualifier attached to a property.
    ///
    /// ## Arguments
    ///
    /// * `prop_ns` and `prop_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `qual_ns` and `qual_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn qualifier(
        &self,
        prop_ns: &str,
        prop_path: &str,
        qual_ns: &str,
        qual_name: &str,
    ) -> Option<XmpValue<String>> {
        if let Some(m) = self.m {
            let c_prop_ns = CString::new(prop_ns).unwrap_or_default();
            let c_prop_name = CString::new(prop_path).unwrap_or_default();
            let c_qual_ns = CString::new(qual_ns).unwrap_or_default();
            let c_qual_name = CString::new(qual_name).unwrap_or_default();

            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                CXmpString::from_ptr(ffi::CXmpMetaGetQualifier(
                    m,
                    &mut err,
                    c_prop_ns.as_ptr(),
                    c_prop_name.as_ptr(),
                    c_qual_ns.as_ptr(),
                    c_qual_name.as_ptr(),
                    &mut options,
                ))
                .map(|value| XmpValue { value, options })
            }
        } else {
            None
        }
    }

    /// Creates or sets a qualifier attached to a property.
    ///
    /// Use this to set a value for an existing qualifier, or create a new
    /// qualifier.
    ///
    /// Use [`XmpMeta::compose_qualifier_path`] to create a complex path.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `prop_name`: See [Accessing
    ///   properties](#accessing-properties). The name of the property to which
    ///   the qualifier is attached.
    /// * `qual_ns` and `qual_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.) Specifies the qualifier.
    /// * `qual_value`: Contains value and flags for the qualifier to be added
    ///   to the property.
    pub fn set_qualifier(
        &mut self,
        namespace: &str,
        prop_name: &str,
        qual_ns: &str,
        qual_name: &str,
        qual_value: &XmpValue<String>,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_struct_ns = CString::new(namespace)?;
            let c_prop_name = CString::new(prop_name.as_bytes())?;
            let c_qual_ns = CString::new(qual_ns)?;
            let c_qual_name = CString::new(qual_name.as_bytes())?;
            let c_qual_value = CString::new(qual_value.value.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetQualifier(
                    m,
                    &mut err,
                    c_struct_ns.as_ptr(),
                    c_prop_name.as_ptr(),
                    c_qual_ns.as_ptr(),
                    c_qual_name.as_ptr(),
                    c_qual_value.as_ptr(),
                    qual_value.options,
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Deletes an XMP subtree rooted at a given qualifier.
    ///
    /// It is not an error if the qualifier does not exist.
    ///
    /// ## Arguments
    ///
    /// * `prop_ns` and `prop_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `qual_ns` and `qual_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    pub fn delete_qualifier(
        &mut self,
        prop_ns: &str,
        prop_name: &str,
        qual_ns: &str,
        qual_name: &str,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_prop_ns = CString::new(prop_ns)?;
            let c_prop_name = CString::new(prop_name.as_bytes())?;
            let c_qual_ns = CString::new(qual_ns)?;
            let c_qual_name = CString::new(qual_name.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaDeleteQualifier(
                    m,
                    &mut err,
                    c_prop_ns.as_ptr(),
                    c_prop_name.as_ptr(),
                    c_qual_ns.as_ptr(),
                    c_qual_name.as_ptr(),
                );
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Retrieves information about a selected item from an alt-text array.
    ///
    /// Localized text properties are stored in alt-text arrays. They allow
    /// multiple concurrent localizations of a property value, for example a
    /// document title or copyright in several languages. These functions
    /// provide convenient support for localized text properties, including a
    /// number of special and obscure aspects. The most important aspect of
    /// these functions is that they select an appropriate array item based on
    /// one or two RFC 3066 language tags. One of these languages, the
    /// "specific" language, is preferred and selected if there is an exact
    /// match. For many languages it is also possible to define a "generic"
    /// language that can be used if there is no specific language match. The
    /// generic language must be a valid RFC 3066 primary subtag, or the empty
    /// string.
    ///
    /// For example, a specific language of `en-US` should be used in the US,
    /// and a specific language of `en-UK` should be used in England. It is also
    /// appropriate to use `en` as the generic language in each case. If a US
    /// document goes to England, the `en-US` title is selected by using the
    /// `en` generic language and the `en-UK` specific language.
    ///
    /// It is considered poor practice, but allowed, to pass a specific language
    /// that is just an RFC 3066 primary tag. For example `en` is not a good
    /// specific language, it should only be used as a generic language. Passing
    /// `i` or `x` as the generic language is also considered poor practice but
    /// allowed.
    ///
    /// Advice from the W3C about the use of RFC 3066 language tags can be found
    /// at <https://www.w3.org/International/articles/language-tags/>.
    ///
    /// **Note:** RFC 3066 language tags must be treated in a case insensitive
    /// manner. The XMP Toolkit does this by normalizing their capitalization:
    ///
    /// * The primary subtag is lower case, the suggested practice of ISO 639.
    /// * All 2-letter secondary subtags are upper case, the suggested practice
    ///   of ISO 3166.
    /// * All other subtags are lower case. The XMP specification defines an
    ///   artificial language, `x-default`, that is used to explicitly denote a
    ///   default item in an alt-text array. The XMP toolkit normalizes alt-text
    ///   arrays such that the x-default item is the first item. The
    ///   [`XmpMeta::set_localized_text`] function has several special features
    ///   related to the `x-default` item. See its description for details. The
    ///   array item is selected according to these rules:
    /// * Look for an exact match with the specific language.
    /// * If a generic language is given, look for a partial match.
    /// * Look for an `x-default` item.
    /// * Choose the first item.
    ///
    /// A partial match with the generic language is where the start of the
    /// item's language matches the generic string and the next character is
    /// `-`. An exact match is also recognized as a degenerate case.
    ///
    /// You can pass `x-default` as the specific language. In this case,
    /// selection of an `x-default` item is an exact match by the first rule,
    /// not a selection by the 3rd rule. The last 2 rules are fallbacks used
    /// when the specific and generic languages fail to produce a match.
    ///
    /// ## Arguments
    ///
    /// * `namespace` and `path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `generic_lang`: The name of the generic language as an RFC 3066
    ///   primary subtag. Can be `None` or the empty string if no generic
    ///   language is wanted.
    /// * `specific_lang`: The name of the specific language as an RFC 3066 tag,
    ///   or `x-default`. Must not be an empty string.
    ///
    /// ## Return value
    ///
    /// If a suitable match is found, returns `Some(XmpValue<String>, String)`
    /// where the second string is the actual language that was matched.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn localized_text(
        &self,
        namespace: &str,
        path: &str,
        generic_lang: Option<&str>,
        specific_lang: &str,
    ) -> Option<(XmpValue<String>, String)> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();
            let c_generic_lang = generic_lang.map(|s| CString::new(s).unwrap_or_default());
            let c_specific_lang = CString::new(specific_lang).unwrap_or_default();

            let mut options: u32 = 0;
            let mut err = ffi::CXmpError::default();

            unsafe {
                let mut c_actual_lang: *const c_char = std::ptr::null_mut();

                CXmpString::from_ptr(ffi::CXmpMetaGetLocalizedText(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    match c_generic_lang {
                        Some(p) => p.as_ptr(),
                        None => std::ptr::null(),
                    },
                    c_specific_lang.as_ptr(),
                    &mut c_actual_lang,
                    &mut options,
                ))
                .map(|value| {
                    (
                        XmpValue { value, options },
                        CXmpString::from_ptr(c_actual_lang).as_string(),
                    )
                })
            }
        } else {
            None
        }
    }

    /// Modifies the value of a selected item in an alt-text array using a
    /// string object.
    ///
    /// Creates an appropriate array item if necessary, and handles special
    /// cases for the `x-default` item.
    ///
    /// The array item is selected according to these rules:
    ///
    /// * Look for an exact match with the specific language.
    /// * If a generic language is given, look for a partial match.
    /// * Look for an `x-default` item.
    /// * Choose the first item.
    ///
    /// A partial match with the generic language is where the start of the
    /// item's language matches the generic string and the next character is
    /// `-`. An exact match is also recognized as a degenerate case.
    ///
    /// You can pass `x-default` as the specific language. In this case,
    /// selection of an `x-default` item is an exact match by the first rule,
    /// not a selection by the 3rd rule. The last 2 rules are fallbacks used
    /// when the specific and generic languages fail to produce a match.
    ///
    /// Item values are modified according to these rules:
    ///
    /// * If the selected item is from a match with the specific language, the
    ///   value of that item is modified. If the existing value of that item
    ///   matches the existing value of the `x-default` item, the `x-default`
    ///   item is also modified. If the array only has 1 existing item (which is
    ///   not `x-default`), an `x-default` item is added with the given value.
    /// * If the selected item is from a match with the generic language and
    ///   there are no other generic matches, the value of that item is
    ///   modified. If the existing value of that item matches the existing
    ///   value of the `x-default` item, the `x-default` item is also modified.
    ///   If the array only has 1 existing item (which is not `x-default`), an
    ///   `x-default` item is added with the given value.
    /// * If the selected item is from a partial match with the generic language
    ///   and there are other partial matches, a new item is created for the
    ///   specific language. The `x-default` item is not modified.
    /// * If the selected item is from the last 2 rules then a new item is
    ///   created for the specific language. If the array only had an
    ///   `x-default` item, the `x-default` item is also modified. If the array
    ///   was empty, items are created for the specific language and
    ///   `x-default`.
    pub fn set_localized_text(
        &mut self,
        namespace: &str,
        path: &str,
        generic_lang: Option<&str>,
        specific_lang: &str,
        item_value: &str,
    ) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_ns = CString::new(namespace).unwrap_or_default();
            let c_name = CString::new(path).unwrap_or_default();
            let c_generic_lang = generic_lang.map(|s| CString::new(s).unwrap_or_default());
            let c_specific_lang = CString::new(specific_lang).unwrap_or_default();
            let c_item_value = CString::new(item_value).unwrap_or_default();

            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetLocalizedText(
                    m,
                    &mut err,
                    c_ns.as_ptr(),
                    c_name.as_ptr(),
                    match c_generic_lang {
                        Some(lang) => lang.as_ptr(),
                        None => std::ptr::null(),
                    },
                    c_specific_lang.as_ptr(),
                    c_item_value.as_ptr(),
                    0,
                );
            };

            XmpError::raise_from_c(&err)?;
            Ok(())
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Composes the path expression for an item in an array.
    ///
    /// ## Arguments
    ///
    /// * `array_ns` and `array_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `item_index`: The index of the desired item. Use
    ///   [`XmpMeta::LAST_ITEM`] to specify the last existing array item.
    ///   **IMPORTANT:** Indices in XMP are 1-based, not zero-based as in most
    ///   of Rust.
    ///
    /// ## Return value
    ///
    /// If successful, the returned string is in the form
    /// `array_name[array_index]`.
    pub fn compose_array_item_path(
        array_ns: &str,
        array_path: &str,
        index: i32,
    ) -> XmpResult<String> {
        let c_array_ns = CString::new(array_ns).unwrap_or_default();
        let c_array_name = CString::new(array_path).unwrap_or_default();

        let mut err = ffi::CXmpError::default();

        unsafe {
            let result = CXmpString::from_ptr(ffi::CXmpMetaComposeArrayItemPath(
                &mut err,
                c_array_ns.as_ptr(),
                c_array_name.as_ptr(),
                index,
            ));

            XmpError::raise_from_c(&err)?;
            Ok(result.as_string())
        }
    }

    /// Composes the path expression to select an alternate item by language.
    ///
    /// Path syntax allows two forms of "content addressing" to select an item
    /// in an array of alternatives. The form used in this function lets you
    /// select an item in an alt-text array based on the value of its
    /// `xml:lang` qualifier. The other form of content addressing is shown
    /// in [`XmpMeta::compose_field_selector`].
    ///
    /// ## Arguments
    ///
    /// * `schema_ns` and `array_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `lang_name`: The RFC 3066 code for the desired language.
    ///
    /// ## Return
    ///
    /// If successful, the returned string is in the form
    /// `schema_ns:array_name[@xml:lang='lang_name']`, where `schema_ns` is the
    /// prefix for the schema namespace.
    ///
    /// This function provides a path expression that is explicitly and only for
    /// a specific language. In most cases,
    /// [`XmpMeta::set_localized_text`] and [`XmpMeta::localized_text`] are
    /// preferred, because they provide extra logic to choose the appropriate
    /// language and maintain consistency with the `x-default` value.
    pub fn compose_lang_selector(
        schema_ns: &str,
        array_name: &str,
        lang_name: &str,
    ) -> XmpResult<String> {
        let c_schema_ns = CString::new(schema_ns).unwrap_or_default();
        let c_array_name = CString::new(array_name).unwrap_or_default();
        let c_lang_name = CString::new(lang_name).unwrap_or_default();

        let mut err = ffi::CXmpError::default();

        unsafe {
            let result = CXmpString::from_ptr(ffi::CXmpMetaComposeLangSelector(
                &mut err,
                c_schema_ns.as_ptr(),
                c_array_name.as_ptr(),
                c_lang_name.as_ptr(),
            ));

            XmpError::raise_from_c(&err)?;
            Ok(result.as_string())
        }
    }

    /// Composes a path expression to select an alternate item by a field's
    /// value.
    ///
    /// Path syntax allows two forms of "content addressing" to select an item
    /// in an array of alternatives. The form used in this function lets you
    /// select an item in an array of structs based on the value of one of the
    /// fields in the structs. The other form of content addressing is shown in
    /// [`XmpMeta::compose_lang_selector`].
    ///
    /// ## Arguments
    ///
    /// * `schema_ns` and `struct_name`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name`: See [Accessing
    ///   properties](#accessing-properties) again.
    /// * `field_value`: The desired value of the field, specified as a string
    ///   object.
    ///
    /// ## Example
    ///
    /// Consider a simple struct that has two fields, the name of a city and the
    /// URI of an FTP site in that city. Use this to create an array of download
    /// alternatives. You can show the user a popup built from the values of the
    /// city fields, then get the corresponding URI as follows:
    ///
    /// ```
    /// use xmp_toolkit::{XmpMeta, XmpResult};
    ///
    /// # fn main() -> XmpResult<()> {
    /// XmpMeta::register_namespace("ns:struct_example/", "struct")?;
    /// XmpMeta::register_namespace("ns:field_example/", "field")?;
    ///
    /// let meta = XmpMeta::default();
    ///
    /// let path = XmpMeta::compose_field_selector(
    ///     "ns:struct_example/",
    ///     "Downloads",
    ///     "ns:field_example/",
    ///     "City",
    ///     Some("Seattle"),
    /// )?;
    /// assert_eq!(path, "Downloads[field:City=\"Seattle\"]");
    ///
    /// println!(
    ///     "field {} exists: {}",
    ///     path,
    ///     meta.struct_field("ns:struct_example/", &path, "ns:field_example/", "URI")
    ///         .is_some()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Return value
    ///
    /// If successful, the returned string is in the form
    /// `schema_ns:struct_name[field_ns:field_name='field_value'].
    pub fn compose_field_selector(
        schema_ns: &str,
        struct_name: &str,
        field_ns: &str,
        field_name: &str,
        field_value: Option<&str>,
    ) -> XmpResult<String> {
        let c_schema_ns = CString::new(schema_ns).unwrap_or_default();
        let c_struct_name = CString::new(struct_name).unwrap_or_default();
        let c_field_ns = CString::new(field_ns).unwrap_or_default();
        let c_field_name = CString::new(field_name).unwrap_or_default();
        let c_field_value = CString::new(field_value.unwrap_or_default()).unwrap_or_default();

        let mut err = ffi::CXmpError::default();

        unsafe {
            let result = CXmpString::from_ptr(ffi::CXmpMetaComposeFieldSelector(
                &mut err,
                c_schema_ns.as_ptr(),
                c_struct_name.as_ptr(),
                c_field_ns.as_ptr(),
                c_field_name.as_ptr(),
                if c_field_value == CString::default() {
                    std::ptr::null()
                } else {
                    c_field_value.as_ptr()
                },
            ));

            XmpError::raise_from_c(&err)?;
            Ok(result.as_string())
        }
    }

    /// Composes the path expression for a qualifier.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns` and `prop_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `qual_ns` and `qual_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Return value
    ///
    /// If successful, the returned string is in the form
    /// `schema_ns:prop_name/?qual_ns:qual_name`.
    pub fn compose_qualifier_path(
        schema_ns: &str,
        prop_path: &str,
        qual_ns: &str,
        qual_name: &str,
    ) -> XmpResult<String> {
        let c_schema_ns = CString::new(schema_ns).unwrap_or_default();
        let c_prop_name = CString::new(prop_path).unwrap_or_default();
        let c_qual_ns = CString::new(qual_ns).unwrap_or_default();
        let c_qual_name = CString::new(qual_name).unwrap_or_default();

        let mut err = ffi::CXmpError::default();

        unsafe {
            let result = CXmpString::from_ptr(ffi::CXmpMetaComposeQualifierPath(
                &mut err,
                c_schema_ns.as_ptr(),
                c_prop_name.as_ptr(),
                c_qual_ns.as_ptr(),
                c_qual_name.as_ptr(),
            ));

            XmpError::raise_from_c(&err)?;
            Ok(result.as_string())
        }
    }

    /// Composes the path expression for a field in a struct.
    ///
    /// ## Arguments
    ///
    /// * `struct_ns` and `struct_path`: See [Accessing
    ///   properties](#accessing-properties).
    /// * `field_ns` and `field_name` take the same form (i.e. see [Accessing
    ///   properties](#accessing-properties) again.)
    ///
    /// ## Return value
    ///
    /// If successful, the returned string is in the form
    /// `struct_ns:struct_name/field_ns:field_name`.
    pub fn compose_struct_field_path(
        struct_ns: &str,
        struct_path: &str,
        field_ns: &str,
        field_name: &str,
    ) -> XmpResult<String> {
        let c_struct_ns = CString::new(struct_ns).unwrap_or_default();
        let c_struct_name = CString::new(struct_path).unwrap_or_default();
        let c_field_ns = CString::new(field_ns).unwrap_or_default();
        let c_field_name = CString::new(field_name).unwrap_or_default();

        let mut err = ffi::CXmpError::default();

        unsafe {
            let result = CXmpString::from_ptr(ffi::CXmpMetaComposeStructFieldPath(
                &mut err,
                c_struct_ns.as_ptr(),
                c_struct_name.as_ptr(),
                c_field_ns.as_ptr(),
                c_field_name.as_ptr(),
            ));

            XmpError::raise_from_c(&err)?;
            Ok(result.as_string())
        }
    }

    /// Sorts the data model tree of an XMP object.
    ///
    /// Use this function to sort the data model of an XMP object into a
    /// canonical order. This can be convenient when comparing data models,
    /// (e.g. by text comparison of `{:#?}` output).
    ///
    /// At the top level the namespaces are sorted by their prefixes. Within a
    /// namespace, the top level properties are sorted by name. Within a struct,
    /// the fields are sorted by their qualified name, i.e. their XML
    /// `prefix:local` form. Unordered arrays of simple items are sorted by
    /// value. Language Alternative arrays are sorted by the `xml:lang`
    /// qualifiers, with the `x-default` item placed first.
    ///
    /// If this function is not called, the data model will typically appear
    /// in order of construction. In other words, content parsed from a file
    /// or string will appear in the order that it did in the source material.
    /// Properties added subsequently will generally be appended in the order of
    /// addition within each container.
    pub fn sort(&mut self) -> XmpResult<()> {
        if let Some(m) = self.m {
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSort(m, &mut err);
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Returns the client-assigned name of this XMP object.
    ///
    /// This name is the empty string by default.
    ///
    /// See also [`XmpMeta::set_name`].
    pub fn name(&self) -> String {
        if let Some(m) = self.m {
            let mut err = ffi::CXmpError::default();
            unsafe { CXmpString::from_ptr(ffi::CXmpMetaGetObjectName(m, &mut err)).as_string() }
        } else {
            String::default()
        }
    }

    /// Assigns a name to this XMP object.
    ///
    /// This name can be retrieved via [`XmpMeta::name`].
    ///
    /// This name is for client use only and it not interpreted by
    /// the XMP Toolkit.
    pub fn set_name(&mut self, name: &str) -> XmpResult<()> {
        if let Some(m) = self.m {
            let c_name = CString::new(name.as_bytes())?;
            let mut err = ffi::CXmpError::default();

            unsafe {
                ffi::CXmpMetaSetObjectName(m, &mut err, c_name.as_ptr());
            }

            XmpError::raise_from_c(&err)
        } else {
            Err(no_cpp_toolkit())
        }
    }

    /// Creates a new `XmpMeta` struct and populates it with metadata from a
    /// string containing serialized RDF. This string must be a complete RDF
    /// parse stream.
    pub fn from_str_with_options(s: &str, options: FromStrOptions) -> XmpResult<Self> {
        let mut err = ffi::CXmpError::default();
        let bytes = s.as_bytes();

        let m = unsafe {
            ffi::CXmpMetaParseFromBuffer(
                &mut err,
                bytes.as_ptr(),
                bytes.len() as u32,
                options.options,
            )
        };

        XmpError::raise_from_c(&err)?;

        let result = XmpMeta { m: Some(m) };

        if options.options & 0x01 != 0 {
            // Caller has asked that we require an `<x:xmpmeta>` element
            // when parsing this XMP payload. If no such element is found,
            // the C++ XMP Toolkit will "succeed" and return an `SXMPMeta`
            // object with no content. In Rust, we translate that to
            // an error condition signaling that the `<x:xmpmeta>` element
            // was missing.

            let mut prop_iter = result.iter(IterOptions::default());
            if prop_iter.next().is_none() {
                return Err(XmpError {
                    error_type: XmpErrorType::XmpMetaElementMissing,
                    debug_message: "x:xmpmeta element not found".to_owned(),
                });
            }
        }

        Ok(result)
    }

    /// Converts metadata in this XMP object into a string as RDF.
    ///
    /// In many cases, this struct's implementation of [`Display`]
    /// will provide reasonable default behavior. (In other words,
    /// you can often call `xmp.to_string()` or include an `XmpMeta`
    /// object directly in a format string).
    ///
    /// Use this function, together with [`ToStringOptions`] if you
    /// need more control over output formats.
    ///
    /// [`Display`]: std::fmt::Display
    pub fn to_string_with_options(&self, options: ToStringOptions) -> XmpResult<String> {
        if let Some(m) = self.m {
            let c_newline = CString::new(options.newline).unwrap_or_default();
            let c_indent = CString::new(options.indent).unwrap_or_default();

            let mut err = ffi::CXmpError::default();

            unsafe {
                let result = CXmpString::from_ptr(ffi::CXmpMetaSerializeToBuffer(
                    m,
                    &mut err,
                    options.options,
                    options.padding,
                    c_newline.as_ptr(),
                    c_indent.as_ptr(),
                    options.base_indent,
                ));

                XmpError::raise_from_c(&err)?;

                Ok(result.as_string())
            }
        } else {
            Err(no_cpp_toolkit())
        }
    }
}

impl<'a> XmpMeta {
    /// Returns an iterator over the schema and properties within an XMP object.
    ///
    /// The top of the XMP data tree is a single root node. This does not
    /// explicitly in an iteration.
    ///
    /// Beneath the root are schema nodes; these collect the top-level
    /// properties in the same namespace. They are created and destroyed
    /// implicitly.
    ///
    /// Beneath the schema nodes are the property nodes. The nodes below a
    /// property node depend on its type (simple, struct, or array) and whether
    /// it has qualifiers.
    ///
    /// The [`IterOptions`] struct defines a starting point for the iteration,
    /// and options that control how it proceeds. By default, iteration starts
    /// at the root and visits all nodes beneath it in a depth-first manner. The
    /// root node iteself is not visited; the first visited node is a schema
    /// node. You can provide a schema name or property path to select a
    /// different starting node. By default, this visits the named root node
    /// first then all nodes beneath it in a depth-first manner.
    pub fn iter(&'a self, options: IterOptions) -> XmpIterator<'a> {
        XmpIterator::new(self, options)
    }
}

impl Clone for XmpMeta {
    /// Returns a deep copy of the XMP metadata packet.
    ///
    /// In the unlikely event of a C++ error reported from the
    /// underlying C++ XMP Toolkit operation, this function will
    /// fail silently and generate an empty XMP data model.
    fn clone(&self) -> Self {
        if let Some(m) = self.m {
            let mut err = ffi::CXmpError::default();
            let m = unsafe { ffi::CXmpMetaClone(m, &mut err) };
            if m.is_null() {
                Self { m: None }
            } else {
                Self { m: Some(m) }
            }
        } else {
            Self { m: None }
        }
    }
}

impl fmt::Debug for XmpMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(m) = self.m {
            let mut result = String::default();

            unsafe {
                let result: *mut String = &mut result;
                ffi::CXmpMetaDumpObj(
                    m,
                    std::mem::transmute::<*mut String, *mut c_void>(result),
                    ffi::xmp_dump_to_string,
                );
            }

            if result.starts_with("Dumping ") {
                result.replace_range(0..8, "");
            }

            write!(f, "{}", result)
        } else {
            write!(f, "(C++ XMP Toolkit unavailable)")
        }
    }
}

impl fmt::Display for XmpMeta {
    /// Convert the XMP data model to RDF using a compact formatting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.to_string_with_options(
            ToStringOptions::default()
                .omit_packet_wrapper()
                .omit_all_formatting(),
        ) {
            Ok(s) => write!(f, "{}", s.trim_end()),
            Err(err) => write!(f, "ERROR ({:#?}): {}", err.error_type, err.debug_message),
        }
    }
}

impl Default for XmpMeta {
    fn default() -> Self {
        let mut err = ffi::CXmpError::default();
        let m = unsafe { ffi::CXmpMetaNew(&mut err) };
        if m.is_null() {
            XmpMeta { m: None }
        } else {
            XmpMeta { m: Some(m) }
        }
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
    /// * `s`: XMP string to be read
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut err = ffi::CXmpError::default();
        let bytes = s.as_bytes();
        let m = unsafe {
            ffi::CXmpMetaParseFromBuffer(&mut err, bytes.as_ptr(), bytes.len() as u32, 0)
        };
        XmpError::raise_from_c(&err)?;

        Ok(XmpMeta { m: Some(m) })
    }
}

/// Per _XMP Toolkit SDK Programmer's Guide_, section _Multi-threading in the
/// API:_
///
/// > The functions in XMPCore and XMPFiles are thread safe. You must call
/// > the initialization and termination functions in a single-threaded manner;
/// > between those calls, you can use threads freely, following a multi-read,
/// > single-writer locking model. All locking is automatic and transparent.
unsafe impl Send for XmpMeta {}

/// An iterator that provides access to items within a property array.
///
/// Create via [`XmpMeta::property_array`].
pub struct ArrayProperty<'a> {
    meta: &'a XmpMeta,
    ns: CString,
    name: CString,
    index: i32,
}

impl<'a> Iterator for ArrayProperty<'a> {
    type Item = XmpValue<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(m) = self.meta.m {
            unsafe {
                let mut options: u32 = 0;
                let mut err = ffi::CXmpError::default();

                self.index += 1;

                CXmpString::from_ptr(ffi::CXmpMetaGetArrayItem(
                    m,
                    &mut err,
                    self.ns.as_ptr(),
                    self.name.as_ptr(),
                    self.index,
                    &mut options,
                ))
                .map(|value| XmpValue { value, options })
            }
        } else {
            None
        }
    }
}

pub(crate) fn no_cpp_toolkit() -> XmpError {
    XmpError {
        error_type: XmpErrorType::NoCppToolkit,
        debug_message: "C++ XMP Toolkit not available".to_owned(),
    }
}

/// Provides options for configuring the XMP parsing behavior
/// provided by [`XmpMeta::from_str_with_options`].
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct FromStrOptions {
    pub(crate) options: u32,
}

impl FromStrOptions {
    pub(crate) const REQUIRE_XMP_META: u32 = 0x0001;
    // pub(crate) const PARSE_MORE_BUFFERS: u32 = 0x0002;
    pub(crate) const STRICT_ALIASING: u32 = 0x0004;

    /// Require a surrounding `x:xmpmeta` element.
    pub fn require_xmp_meta(mut self) -> Self {
        self.options |= Self::REQUIRE_XMP_META;
        self
    }

    /// Do not reconcile alias differences. If differences are found,
    /// return an `Err` result.
    pub fn strict_aliasing(mut self) -> Self {
        self.options |= Self::STRICT_ALIASING;
        self
    }
}

/// Provides options for configuring the XMP serialization behavior
/// provided by [`XmpMeta::to_string_with_options`].
///
/// Note that the Rust XMP Toolkit only provides UTF-8 string encodings.
/// No API is provided for accessing UTF-16 or UTF-32 string encodings.
///
/// We would welcome a PR that adds UTF-16 or UTF-32 encoding if you need
/// it, but we have no plans to implement this ourselves.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ToStringOptions {
    pub(crate) padding: u32,
    pub(crate) newline: String,
    pub(crate) indent: String,
    pub(crate) base_indent: u32,
    pub(crate) options: u32,
}

impl ToStringOptions {
    pub(crate) const EXACT_PACKET_LENGTH: u32 = 0x0200;
    pub(crate) const INCLUDE_RDF_HASH: u32 = 0x2000;
    pub(crate) const INCLUDE_THUMBNAIL_PAD: u32 = 0x0100;
    pub(crate) const OMIT_ALL_FORMATTING: u32 = 0x0800;
    pub(crate) const OMIT_PACKET_WRAPPER: u32 = 0x0010;
    pub(crate) const OMIT_XMP_META_ELEMENT: u32 = 0x1000;
    pub(crate) const READ_ONLY_PACKET: u32 = 0x0020;
    pub(crate) const USE_CANONICAL_FORMAT: u32 = 0x0080;
    pub(crate) const USE_COMPACT_FORMAT: u32 = 0x0040;

    // NOTE: Not exposing API for non-UTF8 serializations for now.

    /// Set the amount of padding to be added if a writeable XML packet is
    /// created.
    ///
    /// If zero or this function is not called, an appropriate amount of padding
    /// is computed.
    pub fn set_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the string to be used as a line terminator.
    ///
    /// If empty or this function is not called, defaults to
    /// linefeed, U+000A, the standard XML newline.
    pub fn set_newline(mut self, newline: String) -> Self {
        self.newline = newline;
        self
    }

    /// Set the string to be used for each level of indentation in the
    /// serialized RDF.
    ///
    /// If empty or this function is not called, defaults to two ASCII spaces,
    /// U+0020.
    pub fn set_indent_string(mut self, indent: String) -> Self {
        self.indent = indent;
        self
    }

    /// Set the number of levels of indentation to be used for the outermost XML
    /// element in the serialized RDF. This is convenient when embedding the
    /// RDF in other text.
    ///
    /// If this function is not called, the outermost XML element will have
    /// no indentation applied.
    pub fn set_base_indent(mut self, base_indent: u32) -> Self {
        self.base_indent = base_indent;
        self
    }

    /// Do not include an XML packet wrapper.
    ///
    /// This can not be specified together with
    /// [`ToStringOptions::read_only_packet`],
    /// [`ToStringOptions::include_thumbnail_pad`], or
    /// [`ToStringOptions::exact_packet_length`].
    pub fn omit_packet_wrapper(mut self) -> Self {
        self.options |= Self::OMIT_PACKET_WRAPPER;
        self
    }

    /// Create a read-only XML packet wapper.
    ///
    /// This can not be specified together with
    /// [`ToStringOptions::omit_packet_wrapper`].
    pub fn read_only_packet(mut self) -> Self {
        self.options |= Self::READ_ONLY_PACKET;
        self
    }

    /// Use a highly compact RDF syntax and layout.
    pub fn use_compact_format(mut self) -> Self {
        self.options |= Self::USE_COMPACT_FORMAT;
        self
    }

    /// Use a canonical form of RDF.
    pub fn use_canonical_format(mut self) -> Self {
        self.options |= Self::USE_CANONICAL_FORMAT;
        self
    }

    /// Include typical space for a JPEG thumbnail in the padding if
    /// no `xmp:Thumbnails` property is present.
    ///
    /// This can not be specified together with
    /// [`ToStringOptions::omit_packet_wrapper`].
    pub fn include_thumbnail_pad(mut self) -> Self {
        self.options |= Self::INCLUDE_THUMBNAIL_PAD;
        self
    }

    /// The padding parameter provides the overall packet length.
    /// The actual amount of padding is computed. An error is returned
    /// if the packet exceeds this length with no padding.
    ///
    /// This can not be specified together with
    /// [`ToStringOptions::omit_packet_wrapper`].
    pub fn exact_packet_length(mut self) -> Self {
        self.options |= Self::EXACT_PACKET_LENGTH;
        self
    }

    /// Omit all formatting whitespace.
    pub fn omit_all_formatting(mut self) -> Self {
        self.options |= Self::OMIT_ALL_FORMATTING;
        self
    }

    /// Omit the `x:xmpmeta` element surrounding the `rdf:RDF` element.
    pub fn omit_xmp_meta_element(mut self) -> Self {
        self.options |= Self::OMIT_XMP_META_ELEMENT;
        self
    }

    /// Include a rdf `Hash` and `Merged` flag in `x:xmpmeta` element.
    pub fn include_rdf_hash(mut self) -> Self {
        self.options |= Self::INCLUDE_RDF_HASH;
        self
    }
}

/// Describes how a new item should be placed relative to existing
/// items in an array.
///
/// **IMPORTANT:** Indices in XMP are 1-based, unlike Rust where
/// indices are typically 0-based.
///
/// Use with [`XmpMeta::set_array_item`].
pub enum ItemPlacement {
    /// Insert before the item at the specified index.
    InsertBeforeIndex(u32),

    /// Insert after the item at the specified index.
    InsertAfterIndex(u32),

    /// Replace the item currently at the specified index.
    ReplaceItemAtIndex(u32),
}
