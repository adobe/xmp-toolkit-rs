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

use crate::XmpValue;

#[test]
fn new() {
    let v = XmpValue::new("test".to_owned());
    assert_eq!(&v.value, "test");
    assert_eq!(v.options, 0);
}

#[test]
fn from() {
    let s = "foo".to_owned();
    let v = XmpValue::from(s);
    assert_eq!(&v.value, "foo");
    assert_eq!(v.options, 0);
}

#[test]
fn into() {
    let s = "foo".to_owned();
    let v: XmpValue<String> = s.into();
    assert_eq!(&v.value, "foo");
    assert_eq!(v.options, 0);
}

/// Test mapping from C++ XMP Toolkit "options" value to `XmpValue`.
mod options {
    use crate::{xmp_value::xmp_prop, XmpValue};

    #[test]
    fn default() {
        let v = XmpValue::<String>::default();
        assert_eq!(&v.value, "");
        assert_eq!(v.options, 0);

        assert!(v.has_no_flags());
        assert!(!v.is_uri());
        assert!(!v.has_qualifiers());
        assert!(!v.is_qualifier());
        assert!(!v.has_lang());
        assert!(!v.has_type());
        assert!(!v.is_struct());
        assert!(!v.is_array());
        assert!(!v.is_ordered());
        assert!(!v.is_alternate());
        assert!(!v.is_alt_text());
        assert!(!v.is_alias());
        assert!(!v.has_aliases());
        assert!(!v.is_internal());
        assert!(!v.is_stable());
        assert!(!v.is_derived());
    }

    #[test]
    fn is_uri() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_URI,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_uri());
    }

    #[test]
    fn has_qualifiers() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_QUALIFIERS,
        };

        assert!(!v.has_no_flags());
        assert!(v.has_qualifiers());
    }

    #[test]
    fn is_qualifier() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_QUALIFIER,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_qualifier());
    }

    #[test]
    fn has_lang() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_LANG,
        };

        assert!(!v.has_no_flags());
        assert!(v.has_lang());
    }

    #[test]
    fn has_type() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_TYPE,
        };

        assert!(!v.has_no_flags());
        assert!(v.has_type());
    }

    #[test]
    fn is_struct() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_STRUCT,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_struct());
    }

    #[test]
    fn is_array() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_ARRAY,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_array());
    }

    #[test]
    fn is_ordered() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ORDERED,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_ordered());
    }

    #[test]
    fn is_alternate() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ALTERNATE,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_alternate());
    }

    #[test]
    fn is_alt_text() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ALT_TEXT,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_alt_text());
    }

    #[test]
    fn is_alias() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_ALIAS,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_alias());
    }

    #[test]
    fn has_aliases() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_ALIASES,
        };

        assert!(!v.has_no_flags());
        assert!(v.has_aliases());
    }

    #[test]
    fn is_internal() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_INTERNAL,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_internal());
    }

    #[test]
    fn is_stable() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_STABLE,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_stable());
    }

    #[test]
    fn is_derived() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_DERIVED,
        };

        assert!(!v.has_no_flags());
        assert!(v.is_derived());
    }

    #[test]
    fn set_is_uri() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_uri(true);

        assert_eq!(v.options, xmp_prop::VALUE_IS_URI);
    }

    #[test]
    fn set_has_qualifiers() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_has_qualifiers(true);

        assert_eq!(v.options, xmp_prop::HAS_QUALIFIERS);
    }

    #[test]
    fn set_is_qualifier() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_qualifier(true);

        assert_eq!(v.options, xmp_prop::IS_QUALIFIER);
    }

    #[test]
    fn set_has_lang() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_has_lang(true);

        assert_eq!(v.options, xmp_prop::HAS_LANG);
    }

    #[test]
    fn set_has_type() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_has_type(true);

        assert_eq!(v.options, xmp_prop::HAS_TYPE);
    }

    #[test]
    fn set_is_struct() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_struct(true);

        assert_eq!(v.options, xmp_prop::VALUE_IS_STRUCT);
    }

    #[test]
    fn set_is_array() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_array(true);

        assert_eq!(v.options, xmp_prop::VALUE_IS_ARRAY);
    }

    #[test]
    fn set_is_ordered() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_ordered(true);

        assert_eq!(v.options, xmp_prop::ARRAY_IS_ORDERED);
    }

    #[test]
    fn set_is_alternate() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_alternate(true);

        assert_eq!(v.options, xmp_prop::ARRAY_IS_ALTERNATE);
    }

    #[test]
    fn set_is_alt_text() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_alt_text(true);

        assert_eq!(v.options, xmp_prop::ARRAY_IS_ALT_TEXT);
    }

    #[test]
    fn set_is_alias() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_alias(true);

        assert_eq!(v.options, xmp_prop::IS_ALIAS);
    }

    #[test]
    fn set_has_aliases() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_has_aliases(true);

        assert_eq!(v.options, xmp_prop::HAS_ALIASES);
    }

    #[test]
    fn set_is_internal() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_internal(true);

        assert_eq!(v.options, xmp_prop::IS_INTERNAL);
    }

    #[test]
    fn set_is_stable() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_stable(true);

        assert_eq!(v.options, xmp_prop::IS_STABLE);
    }

    #[test]
    fn set_is_derived() {
        let v = XmpValue {
            value: "".to_owned(),
            options: 0,
        }
        .set_is_derived(true);

        assert_eq!(v.options, xmp_prop::IS_DERIVED);
    }

    #[test]
    fn set_doesnt_affect_other_flags() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_URI,
        }
        .set_is_derived(true);

        assert_eq!(v.options, xmp_prop::VALUE_IS_URI | xmp_prop::IS_DERIVED);
    }

    #[test]
    fn clear_is_uri() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_URI,
        }
        .set_is_uri(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_has_qualifiers() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_QUALIFIERS,
        }
        .set_has_qualifiers(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_qualifier() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_QUALIFIER,
        }
        .set_is_qualifier(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_has_lang() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_LANG,
        }
        .set_has_lang(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_has_type() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_TYPE,
        }
        .set_has_type(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_struct() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_STRUCT,
        }
        .set_is_struct(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_array() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_ARRAY,
        }
        .set_is_array(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_ordered() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ORDERED,
        }
        .set_is_ordered(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_alternate() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ALTERNATE,
        }
        .set_is_alternate(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_alt_text() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::ARRAY_IS_ALT_TEXT,
        }
        .set_is_alt_text(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_alias() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_ALIAS,
        }
        .set_is_alias(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_has_aliases() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::HAS_ALIASES,
        }
        .set_has_aliases(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_internal() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_INTERNAL,
        }
        .set_is_internal(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_stable() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_STABLE,
        }
        .set_is_stable(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_is_derived() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::IS_DERIVED,
        }
        .set_is_derived(false);

        assert!(v.has_no_flags());
    }

    #[test]
    fn clear_doesnt_affect_other_flags() {
        let v = XmpValue {
            value: "".to_owned(),
            options: xmp_prop::VALUE_IS_URI | xmp_prop::IS_DERIVED,
        }
        .set_is_derived(false);

        assert_eq!(v.options, xmp_prop::VALUE_IS_URI);
    }
}
