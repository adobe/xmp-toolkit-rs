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
};

use crate::{ffi, OpenFileOptions, XmpDateTime, XmpError, XmpErrorType, XmpFile, XmpResult};

/// The `XmpMeta` struct allows access to the XMP Toolkit core services.
///
/// You can create `XmpMeta` structs from metadata that you construct,
/// or that you obtain from files using the [`XmpFile`] struct.
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
    /// * `namespace_uri`: The URI for the namespace. Must be a
    ///   valid XML URI.
    ///
    /// * `suggested_prefix`: The suggested prefix to be used if
    ///   the URI is not yet registered. Must be a valid XML name.
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

    /// Gets a property value.
    ///
    /// When specifying a namespace and path (in this and all other accessors):
    /// * If a namespace URI is specified, it must be for a registered namespace.
    /// * If the namespace is specified only by a prefix in the property name path,
    ///   it must be a registered prefix.
    /// * If both a URI and path prefix are present, they must be corresponding
    ///   parts of a registered namespace.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI for the property. The URI must be for
    ///   a registered namespace. Must not be an empty string.
    ///
    /// * `prop_name`: The name of the property. Can be a general path expression.
    ///   Must not be an empty string. The first component can be a namespace prefix;
    ///   if present without a `schema_ns` value, the prefix specifies the namespace.
    ///   The prefix must be for a registered namespace, and if a namespace URI is
    ///   specified, must match the registered prefix for that namespace.
    ///
    /// ## Error handling
    ///
    /// Any errors (for instance, empty or invalid namespace or property name)
    /// are ignored; the function will return `None` in such cases.
    pub fn property(&self, schema_ns: &str, prop_name: &str) -> Option<String> {
        let c_ns = CString::new(schema_ns).unwrap();
        let c_name = CString::new(prop_name).unwrap();

        unsafe {
            let c_result = ffi::CXmpMetaGetProperty(self.m, c_ns.as_ptr(), c_name.as_ptr());

            if c_result.is_null() {
                None
            } else {
                Some(CStr::from_ptr(c_result).to_string_lossy().into_owned())
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
    /// * `prop_name`: The name of the property. Can be a general
    ///   path expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &str,
    ) -> XmpResult<()> {
        let c_ns = CString::new(schema_ns).unwrap();
        let c_name = CString::new(prop_name).unwrap();
        let c_value = CString::new(prop_value).unwrap();
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpMetaSetProperty(
                self.m,
                &mut err,
                c_ns.as_ptr(),
                c_name.as_ptr(),
                c_value.as_ptr(),
            );
        }

        XmpError::raise_from_c(&err)
    }

    /// Creates or sets a property value using an [`XmpDateTime`] structure.
    ///
    /// This is the simplest property setter. Use it for top-level
    /// simple properties.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general
    ///   path expression. Must not be an empty string. See [`XmpMeta::property()`]
    ///   for namespace prefix usage.
    ///
    /// * `prop_value`: The new value.
    pub fn set_property_date(
        &mut self,
        schema_ns: &str,
        prop_name: &str,
        prop_value: &XmpDateTime,
    ) {
        let c_ns = CString::new(schema_ns).unwrap();
        let c_name = CString::new(prop_name).unwrap();

        unsafe {
            ffi::CXmpMetaSetPropertyDate(self.m, c_ns.as_ptr(), c_name.as_ptr(), prop_value.dt);
        }
    }

    /// Reports whether a property currently exists.
    ///
    /// ## Arguments
    ///
    /// * `schema_ns`: The namespace URI; see [`XmpMeta::property()`].
    ///
    /// * `prop_name`: The name of the property. Can be a general
    ///   path expression. Must not be an empty string. See [`XmpMeta::property()`]
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
