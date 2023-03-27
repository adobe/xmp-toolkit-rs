// Copyright 2022 Adobe. All rights reserved.
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

use std::{convert::From, fmt::Debug};

/// Describes a single property or item in an array property.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct XmpValue<T: Clone + Debug + Default + PartialEq> {
    /// Core value for this item (typically a `String` or scalar value).
    pub value: T,

    /// Raw bitflags that further describe this type.
    pub(crate) options: u32,
}

/// XMP_PROP_* constant values copied/renamed from XMP_Const.h.
pub(crate) mod xmp_prop {
    pub(crate) const VALUE_IS_URI: u32 = 0x00000002;
    pub(crate) const HAS_QUALIFIERS: u32 = 0x00000010;
    pub(crate) const IS_QUALIFIER: u32 = 0x00000020;
    pub(crate) const HAS_LANG: u32 = 0x00000040;
    pub(crate) const HAS_TYPE: u32 = 0x00000080;
    pub(crate) const VALUE_IS_STRUCT: u32 = 0x00000100;
    pub(crate) const VALUE_IS_ARRAY: u32 = 0x00000200;
    pub(crate) const ARRAY_IS_ORDERED: u32 = 0x00000400;
    pub(crate) const ARRAY_IS_ALTERNATE: u32 = 0x00000800;
    pub(crate) const ARRAY_IS_ALT_TEXT: u32 = 0x00001000;
    pub(crate) const IS_ALIAS: u32 = 0x00010000;
    pub(crate) const HAS_ALIASES: u32 = 0x00020000;
    pub(crate) const IS_INTERNAL: u32 = 0x00040000;
    pub(crate) const IS_STABLE: u32 = 0x00100000;
    pub(crate) const IS_DERIVED: u32 = 0x00200000;
    pub(crate) const IS_SCHEMA_NODE: u32 = 0x80000000;
}

impl<T: Clone + Debug + Default + PartialEq> XmpValue<T> {
    /// Creates a new value with default flags.
    pub fn new(value: T) -> Self {
        Self { value, options: 0 }
    }

    /// Returns `true` if none of the other `is...` or `has...` flags
    /// for this value are true.
    pub fn has_no_flags(&self) -> bool {
        self.options == 0
    }

    /// Returns `true` if the XML string form of this property value is a URI,
    /// meaning it uses the `rdf:resource` attribute.
    ///
    /// This is flagged as "discouraged" in the C++ XMP Toolkit API
    /// documentation.
    pub fn is_uri(&self) -> bool {
        self.options & xmp_prop::VALUE_IS_URI != 0
    }

    /// Set this flag if the XML string form of this property value is a URI,
    /// meaning it uses the `rdf:resource` attribute.
    ///
    /// This is flagged as "discouraged" in the C++ XMP Toolkit API
    /// documentation.
    pub fn set_is_uri(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::VALUE_IS_URI)
    }

    // --- options relating to qualifiers attached to a property ---

    /// Returns `true` if the property has qualifiers, such as `rdf:type`
    /// `xml:lang`.
    pub fn has_qualifiers(&self) -> bool {
        self.options & xmp_prop::HAS_QUALIFIERS != 0
    }

    /// Set this flag if the property has qualifiers, such as `rdf:type`
    /// `xml:lang`.
    pub fn set_has_qualifiers(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::HAS_QUALIFIERS)
    }

    /// Returns `true` if this property is a qualifier for some other property,
    /// such as `rdf:type` and `xml:lang`.
    ///
    /// Qualifiers can have arbitrary structure, and can themselves have
    /// qualifiers. If the qualifier itself has a structured value, this
    /// flag is only set for the top node of the qualifier's subtree.
    pub fn is_qualifier(&self) -> bool {
        self.options & xmp_prop::IS_QUALIFIER != 0
    }

    /// Set this flag if this property is a qualifier for some other property,
    /// such as `rdf:type` and `xml:lang`.
    ///
    /// Qualifiers can have arbitrary structure, and can themselves have
    /// qualifiers. If the qualifier itself has a structured value, this
    /// flag is only set for the top node of the qualifier's subtree.
    pub fn set_is_qualifier(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::IS_QUALIFIER)
    }

    /// Returns `true` if this property has an `xml:lang` qualifier.
    ///
    /// Implies [`has_qualifiers`] will also be true.
    ///
    /// [`has_qualifiers`]: Self::has_qualifiers
    pub fn has_lang(&self) -> bool {
        self.options & xmp_prop::HAS_LANG != 0
    }

    /// Set this flag if this property has an `xml:lang` qualifier.
    ///
    /// Implies [`has_qualifiers`] will also be true.
    ///
    /// [`has_qualifiers`]: Self::has_qualifiers
    pub fn set_has_lang(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::HAS_LANG)
    }

    /// Returns `true` if this property has an `rdf:type` qualifier.
    ///
    /// Implies [`has_qualifiers`] will also be true.
    ///
    /// [`has_qualifiers`]: Self::has_qualifiers
    pub fn has_type(&self) -> bool {
        self.options & xmp_prop::HAS_TYPE != 0
    }

    /// Set this flag if this property has an `rdf:type` qualifier.
    ///
    /// Implies [`has_qualifiers`] will also be true.
    ///
    /// [`has_qualifiers`]: Self::has_qualifiers
    pub fn set_has_type(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::HAS_TYPE)
    }

    // --- options relating to the data structure form ---

    /// Returns `true` if this value is a structure with nested fields.
    pub fn is_struct(&self) -> bool {
        self.options & xmp_prop::VALUE_IS_STRUCT != 0
    }

    /// Set this flag if this value is a structure with nested fields.
    pub fn set_is_struct(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::VALUE_IS_STRUCT)
    }

    /// Returns `true` if this value is an array (RDF alt/bag/seq).
    ///
    /// This may mean the array is ordered or unordered. Use the [`is_ordered`]
    /// query to discern between the two.
    ///
    /// [`is_ordered`]: Self::is_ordered
    pub fn is_array(&self) -> bool {
        self.options & xmp_prop::VALUE_IS_ARRAY != 0
    }

    /// Set this flag if this value is an array (RDF alt/bag/seq).
    ///
    /// This may mean the array is ordered or unordered. Use the [`is_ordered`]
    /// query to discern between the two.
    ///
    /// [`is_ordered`]: Self::is_ordered
    pub fn set_is_array(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::VALUE_IS_ARRAY)
    }

    /// Returns `true` if the item order matters. In other words, this
    /// array has been serialized using an `rdf:Seq` container.
    ///
    /// Implies that `is_array` is also `true`.
    ///
    /// [`is_array`]: Self::is_array
    pub fn is_ordered(&self) -> bool {
        self.options & xmp_prop::ARRAY_IS_ORDERED != 0
    }

    /// Set this flag if the item order matters. In other words, this
    /// array has been serialized using an `rdf:Seq` container.
    ///
    /// Implies that [`is_array`] is also `true`.
    ///
    /// [`is_array`]: Self::is_array
    pub fn set_is_ordered(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::ARRAY_IS_ORDERED)
    }

    /// Returns `true` if the items in this array are alternates. In other
    /// words, this array has been serialized using an `rdf:Alt` container.
    ///
    /// Implies that [`is_array`] and [`is_ordered`] are also `true`.
    ///
    /// [`is_array`]: Self::is_array
    /// [`is_ordered`]: Self::is_ordered
    pub fn is_alternate(&self) -> bool {
        self.options & xmp_prop::ARRAY_IS_ALTERNATE != 0
    }

    /// Set this flag if the items in this array are alternates. In other
    /// words, this array has been serialized using an `rdf:Alt` container.
    ///
    /// Implies that [`is_array`] and [`is_ordered`] are also `true`.
    ///
    /// [`is_array`]: Self::is_array
    /// [`is_ordered`]: Self::is_ordered
    pub fn set_is_alternate(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::ARRAY_IS_ALTERNATE)
    }

    /// Returns `true` if items are localized text. Each array element will be
    /// a simple property with an `xml:lang` attribute.
    ///
    /// Implies `is_alternate` is also true.
    ///
    /// [`is_alternate`]: Self::is_alternate
    pub fn is_alt_text(&self) -> bool {
        self.options & xmp_prop::ARRAY_IS_ALT_TEXT != 0
    }

    /// Set this flag if items are localized text. Each array element will be
    /// a simple property with an `xml:lang` attribute.
    ///
    /// Implies [`is_alternate`] is also true.
    ///
    /// [`is_alternate`]: Self::is_alternate
    pub fn set_is_alt_text(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::ARRAY_IS_ALT_TEXT)
    }

    // -- other miscellaneous options --

    /// Returns `true` if this property is an alias name for another property.
    ///
    /// This is only returned by [`XmpMeta::property`](crate::XmpMeta::property)
    /// and then only if the property name is simple, not a path expression.
    pub fn is_alias(&self) -> bool {
        self.options & xmp_prop::IS_ALIAS != 0
    }

    /// Set this flag if this property is an alias name for another property.
    pub fn set_is_alias(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::IS_ALIAS)
    }

    /// Returns `true` if this property is the base value (actual) for a set
    /// of aliases.
    ///
    /// This is only returned by [`XmpMeta::property`](crate::XmpMeta::property)
    /// and then only if the property name is simple, not a path expression.
    pub fn has_aliases(&self) -> bool {
        self.options & xmp_prop::HAS_ALIASES != 0
    }

    /// Set this flag if this property is the base value (actual) for a set
    /// of aliases.
    pub fn set_has_aliases(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::HAS_ALIASES)
    }

    /// Returns `true` if this property is "owned" by the application,
    /// and should not generally be editable in a UI.
    pub fn is_internal(&self) -> bool {
        self.options & xmp_prop::IS_INTERNAL != 0
    }

    /// Set this flag if this property is "owned" by the application,
    /// and should not generally be editable in a UI.
    pub fn set_is_internal(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::IS_INTERNAL)
    }

    /// Returns `true` if the value of this property is not derived from
    /// the document content.
    pub fn is_stable(&self) -> bool {
        self.options & xmp_prop::IS_STABLE != 0
    }

    /// Set this flag if the value of this property is not derived from
    /// the document content.
    pub fn set_is_stable(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::IS_STABLE)
    }

    /// Returns `true` if the value of this property is derived from the
    /// document content.
    pub fn is_derived(&self) -> bool {
        self.options & xmp_prop::IS_DERIVED != 0
    }

    /// Set this flag if the value of this property is derived from the
    /// document content.
    pub fn set_is_derived(self, value: bool) -> Self {
        self.set_flag(value, xmp_prop::IS_DERIVED)
    }

    fn set_flag(mut self, value: bool, prop_constant: u32) -> Self {
        if value {
            self.options |= prop_constant;
        } else {
            self.options &= !prop_constant;
        }

        self
    }

    /// Returns `true` if this property is an schema node, which is
    /// created implicitly during iteration via [`XmpIterator`].
    ///
    /// [`XmpIterator`]: crate::XmpIterator
    pub fn is_schema_node(&self) -> bool {
        self.options & xmp_prop::IS_SCHEMA_NODE != 0
    }
}

impl<T: Clone + Debug + Default + PartialEq> From<T> for XmpValue<T> {
    fn from(value: T) -> Self {
        Self { value, options: 0 }
    }
}

impl From<&str> for XmpValue<String> {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            options: 0,
        }
    }
}
