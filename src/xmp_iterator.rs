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

use std::{ffi::CString, os::raw::c_char};

use crate::{
    ffi::{self, CXmpString},
    XmpMeta, XmpValue,
};

/// Iterator over an XMP data model or a subset thereof.
///
/// Create via [`XmpMeta::iter`].
pub struct XmpIterator<'a> {
    #[allow(dead_code)]
    pub(crate) m: &'a XmpMeta,
    pub(crate) i: *mut ffi::CXmpIterator,
}

impl<'a> XmpIterator<'a> {
    pub(crate) fn new(meta: &'a XmpMeta, options: IterOptions) -> Self {
        let mut err = ffi::CXmpError::default();

        if let Some(m) = meta.m {
            let c_schema_ns = CString::new(options.schema_ns).unwrap_or_default();
            let c_prop_name = CString::new(options.prop_name).unwrap_or_default();

            unsafe {
                Self {
                    m: meta,
                    i: ffi::CXmpIteratorNew(
                        m,
                        &mut err,
                        c_schema_ns.as_ptr(),
                        c_prop_name.as_ptr(),
                        options.options,
                    ),
                }
            }
        } else {
            Self {
                m: meta,
                i: std::ptr::null_mut(),
            }
        }
    }

    /// Skip the subtree below the current node.
    pub fn skip_subtree(&mut self) {
        if !self.i.is_null() {
            unsafe {
                let mut err = ffi::CXmpError::default();
                ffi::CXmpIteratorSkip(self.i, &mut err, 1);
            }
        }
    }

    /// Skip the subtree below and remaining siblings of the current node.
    pub fn skip_siblings(&mut self) {
        if !self.i.is_null() {
            unsafe {
                let mut err = ffi::CXmpError::default();
                ffi::CXmpIteratorSkip(self.i, &mut err, 2);
            }
        }
    }
}

impl<'a> Drop for XmpIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpIteratorDrop(self.i);
        }
    }
}

impl<'a> Iterator for XmpIterator<'a> {
    type Item = XmpProperty;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.i.is_null() {
            unsafe {
                let mut err = ffi::CXmpError::default();
                let mut c_schema_ns: *const c_char = std::ptr::null_mut();
                let mut c_prop_path: *const c_char = std::ptr::null_mut();
                let mut c_prop_value: *const c_char = std::ptr::null_mut();
                let mut options: u32 = 0;

                if ffi::CXmpIteratorNext(
                    self.i,
                    &mut err,
                    &mut c_schema_ns,
                    &mut c_prop_path,
                    &mut c_prop_value,
                    &mut options,
                ) {
                    Some(XmpProperty {
                        schema_ns: CXmpString::from_ptr(c_schema_ns).as_string(),
                        name: CXmpString::from_ptr(c_prop_path).as_string(),
                        value: XmpValue {
                            value: CXmpString::from_ptr(c_prop_value).as_string(),
                            options,
                        },
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

/// Provides options for configuring the XMP iteration behavior
/// provided by [`XmpMeta::iter`].
///
/// This struct defines a starting point for the iteration, and options that
/// control how it proceeds.
///
/// By default, iteration starts at the root and visits all nodes beneath it in
/// a depth-first manner. The root node iteself is never visited; the first
/// visited node is a schema node. You can provide a schema name or property
/// path to select a different starting node. In those cases, this visits the
/// named root node first then all nodes beneath it in a depth-first manner.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IterOptions {
    pub(crate) schema_ns: String,
    pub(crate) prop_name: String,
    pub(crate) options: u32,
}

impl IterOptions {
    const JUST_CHILDREN: u32 = 0x0100;
    const JUST_LEAF_NAME: u32 = 0x0400;
    const JUST_LEAF_NODES: u32 = 0x0200;
    const OMIT_QUALIFIERS: u32 = 0x1000;

    /// Restrict iteration to those properties that are part of the named
    /// schema.
    pub fn schema_ns(mut self, schema_ns: &str) -> Self {
        self.schema_ns = schema_ns.to_owned();
        self.prop_name = String::default();
        self
    }

    /// Restrict iteration to properties within a specific property.
    pub fn property(mut self, schema_ns: &str, prop_name: &str) -> Self {
        self.schema_ns = schema_ns.to_owned();
        self.prop_name = prop_name.to_owned();
        self
    }

    /// Restrict iteration to immediate children of the root.
    pub fn immediate_children_only(mut self) -> Self {
        self.options |= Self::JUST_CHILDREN;
        self
    }

    /// Restrict iteration to leaf nodes only.
    pub fn leaf_nodes_only(mut self) -> Self {
        self.options |= Self::JUST_LEAF_NODES;
        self
    }

    /// Return only the leaf part of the path.
    pub fn leaf_name_only(mut self) -> Self {
        self.options |= Self::JUST_LEAF_NAME;
        self
    }

    /// Omit all qualifiers.
    pub fn omit_qualifiers(mut self) -> Self {
        self.options |= Self::OMIT_QUALIFIERS;
        self
    }
}

/// Value of a single property found via iterating the XMP data model.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct XmpProperty {
    /// Schema namespace for this property.
    pub schema_ns: String,

    /// XPath name of this property.
    pub name: String,

    /// Value of this property.
    pub value: XmpValue<String>,
}
