// TO DO: Revise API documentation to fit the Rust wrapper.

use std::ffi::{CStr, CString};

use crate::ffi;
use crate::xmp_toolkit::XmpToolkit;

#[allow(dead_code)] // because xmp is never used
pub struct XmpMeta<'xmp> {
    pub(crate) m: *mut ffi::CXmpMeta,
    pub(crate) xmp: &'xmp XmpToolkit,
    // pub(crate) is used because XmpFile::get_xmp
    // can create this struct.
}

impl<'xmp> Drop for XmpMeta<'xmp> {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpMetaDrop(self.m);
        }
    }
}

impl<'xmp> XmpMeta<'xmp> {
    /// Creates a new, empty metadata struct.
    pub fn new(xmp: &'xmp XmpToolkit) -> XmpMeta<'xmp> {
        let m = unsafe { ffi::CXmpMetaNew() };
        XmpMeta { m, xmp }
    }

    /// Registers a namespace URI with a suggested prefix.
    ///
    /// If the URI is not registered but the suggested prefix
    /// is in use, a unique prefix is created from the suggested one.
    /// The actual registered prefix is returned. It is not an error
    /// if the URI is already registered, regardless of the prefix.
    ///
    /// @param namespace_uri The URI for the namespace. Must be a
    /// valid XML URI.
    ///
    /// @param suggested_prefix The suggested prefix to be used if
    /// the URI is not yet registered. Must be a valid XML name.
    ///
    /// Returns the prefix actually registered for this URI.
    ///
    /// @note No checking is done on either the URI or the prefix.
    pub fn register_namespace(
        _xmp: &XmpToolkit,
        namespace_uri: &str,
        suggested_prefix: &str,
    ) -> String {
        let c_ns = CString::new(namespace_uri).unwrap();
        let c_sp = CString::new(suggested_prefix).unwrap();

        unsafe {
            let c_result = ffi::CXmpMetaRegisterNamespace(c_ns.as_ptr(), c_sp.as_ptr());

            CStr::from_ptr(c_result).to_string_lossy().into_owned()
        }
    }

    /// Creates or sets a property value.
    ///
    /// This is the simplest property setter. Use it for top-level
    /// simple properties.
    ///
    /// @param schemaNS The namespace URI; see \c GetProperty().
    ///
    /// @param propName The name of the property. Can be a general
    /// path expression, must not be null or the empty string;
    /// see \c GetProperty() for namespace prefix usage.
    ///
    /// @param propValue The new value, a pointer to a null
    /// terminated UTF-8 string. Must be null for arrays and non-leaf
    /// levels of structs that do not have values.
    pub fn set_property(&mut self, schema_ns: &str, prop_name: &str, prop_value: &str) {
        let c_ns = CString::new(schema_ns).unwrap();
        let c_name = CString::new(prop_name).unwrap();
        let c_value = CString::new(prop_value).unwrap();

        unsafe {
            ffi::CXmpMetaSetProperty(self.m, c_ns.as_ptr(), c_name.as_ptr(), c_value.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::XmpToolkit;

    use super::*;

    #[test]
    fn new_empty() {
        let xmp = XmpToolkit::new();
        let mut _m = XmpMeta::new(&xmp);
    }

    #[test]
    fn register_namespace() {
        let xmp = XmpToolkit::new();
        assert_eq!(
            XmpMeta::register_namespace(&xmp, "http://purl.org/dc/terms/", "dcterms"),
            "dcterms:"
        );
    }
}
