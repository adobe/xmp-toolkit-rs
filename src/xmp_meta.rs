// TO DO: Revise API documentation to fit the Rust wrapper.

use std::ffi::{CStr, CString};

use crate::ffi;
use crate::xmp_date_time::XmpDateTime;

pub struct XmpMeta {
    pub(crate) m: *mut ffi::CXmpMeta,
    // pub(crate) is used because XmpFile::get_xmp
    // can create this struct.
}

impl Drop for XmpMeta {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpMetaDrop(self.m);
        }
    }
}

impl Default for XmpMeta {
    fn default() -> Self {
        XmpMeta::new()
    }
}

impl XmpMeta {
    /// Creates a new, empty metadata struct.
    pub fn new() -> XmpMeta {
        let m = unsafe { ffi::CXmpMetaNew() };
        XmpMeta { m }
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
    pub fn register_namespace(namespace_uri: &str, suggested_prefix: &str) -> String {
        // These .unwrap() calls are deemed unlikely to panic as this
        // function is typically called with known, standardized strings
        // in the ASCII space.
        let c_ns = CString::new(namespace_uri).unwrap();
        let c_sp = CString::new(suggested_prefix).unwrap();

        unsafe {
            let c_result = ffi::CXmpMetaRegisterNamespace(c_ns.as_ptr(), c_sp.as_ptr());

            CStr::from_ptr(c_result).to_string_lossy().into_owned()
        }
    }

    /// Gets a property value.
    ///
    /// This is the simplest property accessor. Use this to retrieve the values of top-level simple
    /// properties, or after using the path composition functions in \c TXMPUtils.
    ///
    /// When specifying a namespace and path (in this and all other accessors):
    ///   \li If a namespace URI is specified, it must be for a registered namespace.
    ///   \li If the namespace is specified only by a prefix in the property name path,
    /// it must be a registered prefix.
    ///   \li If both a URI and path prefix are present, they must be corresponding
    /// parts of a registered namespace.
    ///
    /// @param schemaNS The namespace URI for the property. The URI must be for a registered
    /// namespace. Must not be null or the empty string.
    ///
    /// @param propName The name of the property. Can be a general path expression, must not be null
    /// or the empty string. The first component can be a namespace prefix; if present without a
    /// \c schemaNS value, the prefix specifies the namespace. The prefix must be for a registered
    /// namespace, and if a namespace URI is specified, must match the registered prefix for that
    /// namespace.
    pub fn get_property(&self, schema_ns: &str, prop_name: &str) -> Option<String> {
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

    /// Creates or sets a property value using an \c #XmpDateTime structure..
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

    /// Rreports whether a property currently exists.
    ///
    /// @param schemaNS The namespace URI for the property; see \c GetProperty().
    ///
    /// @param propName The name of the property; see \c GetProperty().
    pub fn does_property_exist(&self, schema_ns: &str, prop_name: &str) -> bool {
        let c_ns = CString::new(schema_ns).unwrap();
        let c_name = CString::new(prop_name).unwrap();

        let r = unsafe { ffi::CXmpMetaDoesPropertyExist(self.m, c_ns.as_ptr(), c_name.as_ptr()) };
        r != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let mut _m = XmpMeta::new();
    }

    #[test]
    fn register_namespace() {
        assert_eq!(
            XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms"),
            "dcterms:"
        );
    }
}
