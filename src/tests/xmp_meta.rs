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

use crate::XmpMeta;

#[test]
fn new_empty() {
    let mut _m = XmpMeta::new();

    // TODO: Add more tests when we can iterate.
}

mod from_file {
    use std::path::PathBuf;

    use crate::{tests::fixtures::*, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        assert_eq!(
            m.property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
                .unwrap(),
            XmpValue {
                value: "Adobe Photoshop CS2 Windows".to_owned(),
                options: 0
            }
        );

        assert_eq!(
            m.property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
                .unwrap(),
            XmpValue {
                value: "Dell 1905FP Color Profile".to_owned(),
                options: 0
            }
        );

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfilx")
            .is_none());
    }

    #[test]
    fn no_xmp() {
        let err = XmpMeta::from_file(fixture_path("no_xmp.txt"))
            .err()
            .unwrap();
        // NOTE: Can't use unwrap_err() because XmpMeta doesn't implement Debug trait.

        assert_eq!(err.error_type, XmpErrorType::Unavailable);
        assert_eq!(err.debug_message, "No XMP in file");
    }

    #[test]
    fn file_not_found() {
        let bad_path = PathBuf::from("doesnotexist.jpg");
        let err = XmpMeta::from_file(&bad_path).err().unwrap();
        // NOTE: Can't use unwrap_err() because XmpMeta doesn't implement Debug trait.

        assert_eq!(err.error_type, XmpErrorType::NoFile);
    }
}

mod from_str {
    use std::str::FromStr;

    use crate::{tests::fixtures::*, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        assert_eq!(
            m.property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
                .unwrap(),
            XmpValue {
                value: "Adobe Photoshop CS2 Windows".to_owned(),
                options: 0
            }
        );

        assert_eq!(
            m.property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
                .unwrap(),
            XmpValue {
                value: "Dell 1905FP Color Profile".to_owned(),
                options: 0
            }
        );

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfilx")
            .is_none());
    }

    #[test]
    fn bad_xmp() {
        // TXMPMeta::ParseFromBuffer doesn't seem to throw exceptions,
        // regardless of how badly-formed the XMP is. This test merely
        // confirms that we pass that behavior through.
        let m = XmpMeta::from_str("this is not XMP").unwrap();

        assert!(m
            .property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
            .is_none());

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
            .is_none());

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfilx")
            .is_none());
    }

    #[test]
    fn empty_string() {
        // TXMPMeta::ParseFromBuffer doesn't seem to throw exceptions,
        // regardless of how badly-formed the XMP is. This test merely
        // confirms that we pass that behavior through.
        let m = XmpMeta::from_str("").unwrap();

        assert!(m
            .property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
            .is_none());

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
            .is_none());

        assert!(m
            .property("http://ns.adobe.com/photoshop/1.0/", "ICCProfilx")
            .is_none());
    }
}

mod register_namespace {
    use crate::{XmpErrorType, XmpMeta};

    #[test]
    fn happy_path() {
        assert_eq!(
            XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap(),
            "dcterms:"
        );
    }

    #[test]
    fn empty_namespace() {
        let err = XmpMeta::register_namespace("", "dcterms").unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty namespace URI");
    }
}

mod contains_property {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta};

    #[test]
    fn exists() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(m.contains_property(xmp_ns::XMP, "CreatorTool"));
    }

    #[test]
    fn doesnt_exist() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.contains_property(xmp_ns::XMP, "RandomProperty"));
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.contains_property("", "CreatorTool"));
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.contains_property(xmp_ns::XMP, ""));
    }
}

mod contains_struct_field {
    use std::str::FromStr;

    use crate::{xmp_ns, XmpMeta};

    const STRUCT_EXAMPLE: &str = r#"
        <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0-c000 1.000000, 0000/00/00-00:00:00">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:Description rdf:about=""
                xmlns:xmp="http://ns.adobe.com/xap/1.0/"
                xmlns:xmpRights="http://ns.adobe.com/xap/1.0/rights/"
                xmlns:Iptc4xmpCore="http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/"
                xmpRights:Marked="True">
                <Iptc4xmpCore:CreatorContactInfo
                    Iptc4xmpCore:CiAdrPcode="98110"
                    Iptc4xmpCore:CiAdrCtry="US"/>
            </rdf:Description>
        </rdf:RDF>
    </x:xmpmeta>
    "#;

    #[test]
    fn exists() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(m.contains_struct_field(
            xmp_ns::IPTC_CORE,
            "CreatorContactInfo",
            xmp_ns::IPTC_CORE,
            "CiAdrPcode"
        ));
    }

    #[test]
    fn doesnt_exist() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(!m.contains_struct_field(
            xmp_ns::IPTC_CORE,
            "CreatorContactInfo",
            xmp_ns::IPTC_CORE,
            "CiAdrPcodx"
        ));
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(!m.contains_struct_field(
            "",
            "CreatorContactInfo",
            xmp_ns::IPTC_CORE,
            "CiAdrPcode"
        ));
    }

    #[test]
    fn empty_struct_name() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(!m.contains_struct_field(xmp_ns::IPTC_CORE, "", xmp_ns::IPTC_CORE, "CiAdrPcode"));
    }
    #[test]
    fn empty_field_namespace() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(!m.contains_struct_field(
            xmp_ns::IPTC_CORE,
            "CreatorContactInfo",
            "",
            "CiAdrPcode"
        ));
    }

    #[test]
    fn empty_field_name() {
        let m = XmpMeta::from_str(STRUCT_EXAMPLE).unwrap();
        assert!(!m.contains_struct_field(
            xmp_ns::IPTC_CORE,
            "CreatorContactInfo",
            xmp_ns::IPTC_CORE,
            ""
        ));
    }
}

mod property {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property(xmp_ns::XMP, "CreatorTool"),
            Some(XmpValue {
                value: "Adobe Photoshop CS2 Windows".to_owned(),
                options: 0
            })
        );
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property(xmp_ns::XMP, "\0"), None);
    }
}

mod property_array {
    use std::str::FromStr;

    use crate::{tests::fixtures::*, XmpMeta, XmpValue};

    #[test]
    fn happy_path_creator_seq() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        let mut creators: Vec<XmpValue<String>> = m
            .property_array("http://purl.org/dc/elements/1.1/", "creator")
            .collect();

        assert_eq!(creators.len(), 1);

        let creator = creators.pop().unwrap();
        assert_eq!(creator.value, "Llywelyn");
        // assert_eq!(creator.options, 0);
        // TO DO: Implement this test when options are exposed.
    }

    #[test]
    fn happy_path_creator_bag() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        let mut subjects: Vec<String> = m
            .property_array("http://purl.org/dc/elements/1.1/", "subject")
            .map(|v| v.value)
            .collect();

        subjects.sort();

        assert_eq!(
            subjects,
            vec!("Stefan", "XMP", "XMPFiles", "purple", "square", "test")
        );
    }

    #[test]
    fn no_such_property() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        let first_creator = m
            .property_array("http://purl.org/dc/elements/1.1/", "creatorx")
            .next();

        assert!(first_creator.is_none());
    }
}

mod property_bool {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_bool(xmp_ns::XMP_RIGHTS, "Marked"),
            Some(XmpValue {
                value: false,
                options: 0
            })
        );
    }

    #[test]
    fn unrecognizable_as_bool() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_bool(xmp_ns::XMP, "CreatorTool"), None);
    }

    #[test]
    fn value_1_is_true() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_bool(xmp_ns::TIFF, "Orientation"),
            Some(XmpValue {
                value: true,
                options: 0
            })
        );
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_bool("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_bool(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_bool("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_bool(xmp_ns::XMP, "\0"), None);
    }
}

mod property_i32 {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_i32(xmp_ns::EXIF, "PixelXDimension"),
            Some(XmpValue {
                value: 200,
                options: 0
            })
        );
    }

    #[test]
    fn unrecognizable_as_int() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32(xmp_ns::XMP, "CreatorTool"), None);
    }

    #[test]
    fn bool_value() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32(xmp_ns::XMP_RIGHTS, "Marked"), None);
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i32(xmp_ns::XMP, "\0"), None);
    }
}

mod property_i64 {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_i64(xmp_ns::EXIF, "PixelXDimension"),
            Some(XmpValue {
                value: 200,
                options: 0
            })
        );
    }

    #[test]
    fn unrecognizable_as_int() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64(xmp_ns::XMP, "CreatorTool"), None);
    }

    #[test]
    fn bool_value() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64(xmp_ns::XMP_RIGHTS, "Marked"), None);
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_i64(xmp_ns::XMP, "\0"), None);
    }
}

mod property_f64 {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_f64(xmp_ns::EXIF, "PixelXDimension"),
            Some(XmpValue {
                value: 200.0,
                options: 0
            })
        );
    }

    #[test]
    fn ratio() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64(xmp_ns::TIFF, "XResolution"), None);
    }

    #[test]
    fn unrecognizable_as_float() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64(xmp_ns::XMP, "CreatorTool"), None);
    }

    #[test]
    fn bool_value() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64(xmp_ns::XMP_RIGHTS, "Marked"), None);
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_f64(xmp_ns::XMP, "\0"), None);
    }
}

mod property_date {
    use crate::{
        tests::fixtures::*, xmp_ns, XmpDate, XmpDateTime, XmpMeta, XmpTime, XmpTimeZone, XmpValue,
    };

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property_date(xmp_ns::XMP, "ModifyDate"),
            Some(XmpValue {
                value: XmpDateTime {
                    date: Some(XmpDate {
                        year: 2006,
                        month: 4,
                        day: 27
                    }),
                    time: Some(XmpTime {
                        hour: 15,
                        minute: 38,
                        second: 36,
                        nanosecond: 655000000,
                        time_zone: Some(XmpTimeZone { hour: 2, minute: 0 }),
                    })
                },
                options: 0
            })
        );
    }

    #[test]
    fn unrecognizable_as_date() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date(xmp_ns::XMP, "CreatorTool"), None);
    }

    #[test]
    fn bool_value() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date(xmp_ns::XMP_RIGHTS, "Marked"), None);
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date("", "CreatorTool"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date(xmp_ns::XMP, ""), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date("\0", "CreatorTool"), None);
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(m.property_date(xmp_ns::XMP, "\0"), None);
    }
}

mod set_property {
    use crate::{tests::fixtures::*, xmp_value::xmp_prop, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        m.set_property("http://purl.org/dc/terms/", "provenance", &"blah".into())
            .unwrap();

        assert_eq!(
            m.property("http://purl.org/dc/terms/", "provenance")
                .unwrap(),
            XmpValue {
                value: "blah".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        m.set_property(
            "http://purl.org/dc/terms/",
            "provenance",
            &XmpValue::<String>::from("blah").set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property("http://purl.org/dc/terms/", "provenance")
                .unwrap(),
            XmpValue {
                value: "blah".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let err = m
            .set_property("http://purl.org/dc/terms/", "", &"blah".into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadXPath);
        assert_eq!(err.debug_message, "Empty property name");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let err = m
            .set_property("http://purl.org/dc/terms/", "x\0x", &"blah".into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod set_property_bool {
    use crate::{tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_bool(xmp_ns::XMP_RIGHTS, "Marked", &true.into())
            .unwrap();

        assert_eq!(
            m.property(xmp_ns::XMP_RIGHTS, "Marked").unwrap(),
            XmpValue {
                value: "True".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_bool(
            xmp_ns::XMP_RIGHTS,
            "Marked",
            &XmpValue::from(true).set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property(xmp_ns::XMP_RIGHTS, "Marked").unwrap(),
            XmpValue {
                value: "True".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m.set_property_bool("", "Marked", &true.into()).unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_bool("x\0x", "Marked", &true.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod set_property_i32 {
    use crate::{tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_i32(xmp_ns::EXIF, "PixelXDimension", &225.into())
            .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_i32(
            xmp_ns::EXIF,
            "PixelXDimension",
            &XmpValue::from(225).set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_i32("", "PixelXDimension", &225.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_i32("x\0x", "PixelXDimension", &225.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod set_property_i64 {
    use crate::{tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_i64(xmp_ns::EXIF, "PixelXDimension", &225.into())
            .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_i64(
            xmp_ns::EXIF,
            "PixelXDimension",
            &XmpValue::from(225).set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_i64("", "PixelXDimension", &225.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_i64("x\0x", "PixelXDimension", &225.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod set_property_f64 {
    use crate::{tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, XmpErrorType, XmpMeta, XmpValue};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_f64(xmp_ns::EXIF, "PixelXDimension", &225.7.into())
            .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225.700000".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        m.set_property_f64(
            xmp_ns::EXIF,
            "PixelXDimension",
            &XmpValue::from(225.7).set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property(xmp_ns::EXIF, "PixelXDimension").unwrap(),
            XmpValue {
                value: "225.700000".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_f64("", "PixelXDimension", &225.7.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        let err = m
            .set_property_f64("x\0x", "PixelXDimension", &225.7.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod set_property_date {
    use crate::{
        tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, XmpDate, XmpDateTime, XmpErrorType,
        XmpMeta, XmpTime, XmpTimeZone, XmpValue,
    };

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 20,
                minute: 48,
                second: 4,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        m.set_property_date(xmp_ns::XMP, "MetadataDate", &updated_time.into())
            .unwrap();

        assert_eq!(
            m.property(xmp_ns::XMP, "MetadataDate").unwrap(),
            XmpValue {
                value: "2022-10-19T20:48:04.000000042-07:00".to_owned(),
                options: 0
            }
        );
    }

    #[test]
    fn options() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 20,
                minute: 48,
                second: 4,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        m.set_property_date(
            xmp_ns::XMP,
            "MetadataDate",
            &XmpValue::from(updated_time).set_is_uri(true),
        )
        .unwrap();

        assert_eq!(
            m.property(xmp_ns::XMP, "MetadataDate").unwrap(),
            XmpValue {
                value: "2022-10-19T20:48:04.000000042-07:00".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            }
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime::current().unwrap();

        let err = m
            .set_property_date("", "MetadataDate", &updated_time.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime::current().unwrap();

        let err = m
            .set_property_date("x\0x", "MetadataDate", &updated_time.into())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod localized_text {
    use std::str::FromStr;

    use crate::{xmp_ns, xmp_value::xmp_prop, XmpMeta};

    const LOCALIZED_TEXT_EXAMPLE: &str = r#"<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
            xmlns:dc="http://purl.org/dc/elements/1.1/">
            <rdf:Description rdf:about="">
                <dc:title>
                    <rdf:Alt>
                        <rdf:li xml:lang="x-default">
                            XMP - Extensible Metadata Platform
                        </rdf:li>
                        <rdf:li xml:lang="en-us">
                            XMP - Extensible Metadata Platform (US English)
                        </rdf:li>
                        <rdf:li xml:lang="fr">
                            XMP - Une Platforme Extensible pour les Métadonnées
                        </rdf:li>
                    </rdf:Alt>
                </dc:title>
            </rdf:Description>
        </rdf:RDF>"#;

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();

        let (value, actual_lang) = m
            .localized_text(xmp_ns::DC, "title", None, "x-default")
            .unwrap();

        assert_eq!(value.value.trim(), "XMP - Extensible Metadata Platform");
        assert_eq!(value.options, xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS);
        assert_eq!(actual_lang, "x-default");

        let (value, actual_lang) = m
            .localized_text(xmp_ns::DC, "title", Some("x-default"), "x-default")
            .unwrap();

        assert_eq!(value.value.trim(), "XMP - Extensible Metadata Platform");
        assert_eq!(value.options, xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS);
        assert_eq!(actual_lang, "x-default");

        let (value, actual_lang) = m
            .localized_text(xmp_ns::DC, "title", Some("en"), "en-US")
            .unwrap();

        assert_eq!(
            value.value.trim(),
            "XMP - Extensible Metadata Platform (US English)"
        );
        assert_eq!(value.options, xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS);
        assert_eq!(actual_lang, "en-US");

        let (value, actual_lang) = m
            .localized_text(xmp_ns::DC, "title", Some("en-us"), "en-uk")
            .unwrap();

        assert_eq!(value.value.trim(), "XMP - Extensible Metadata Platform");
        assert_eq!(value.options, xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS);
        assert_eq!(actual_lang, "x-default");

        let (value, actual_lang) = m
            .localized_text(xmp_ns::DC, "title", Some("fr"), "fr")
            .unwrap();

        assert_eq!(
            value.value.trim(),
            "XMP - Une Platforme Extensible pour les Métadonnées"
        );
        assert_eq!(value.options, xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS);
        assert_eq!(actual_lang, "fr");
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(m.localized_text("", "CreatorTool", None, "x-default"), None);
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(m.localized_text(xmp_ns::XMP, "", None, "x-default"), None);
    }

    #[test]
    fn invalid_namespace() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(
            m.localized_text("\0", "CreatorTool", None, "x-default"),
            None,
        );
    }

    #[test]
    fn invalid_prop_name() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(m.localized_text(xmp_ns::XMP, "\0", None, "x-default"), None);
    }

    #[test]
    fn invalid_generic_lang() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(
            m.localized_text(xmp_ns::XMP, "title", Some("no-such-lang"), "x-default"),
            None
        );
    }

    #[test]
    fn invalid_specific_lang() {
        let m = XmpMeta::from_str(LOCALIZED_TEXT_EXAMPLE).unwrap();
        assert_eq!(
            m.localized_text(xmp_ns::XMP, "title", Some("x-default"), "no-such-lang"),
            None
        );
    }
}

mod compose_struct_field_path {
    use crate::{xmp_ns, XmpMeta};

    #[test]
    fn happy_path() {
        assert_eq!(
            XmpMeta::compose_struct_field_path(xmp_ns::XMP, "StructName", xmp_ns::XMP, "FieldName")
                .unwrap(),
            "StructName/xmp:FieldName"
        );
    }
}
