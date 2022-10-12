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

    use crate::{tests::fixtures::*, XmpErrorType, XmpMeta};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        assert_eq!(
            m.property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
                .unwrap(),
            "Adobe Photoshop CS2 Windows"
        );

        assert_eq!(
            m.property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
                .unwrap(),
            "Dell 1905FP Color Profile"
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

    use crate::{tests::fixtures::*, XmpMeta};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        assert_eq!(
            m.property("http://ns.adobe.com/xap/1.0/", "CreatorTool")
                .unwrap(),
            "Adobe Photoshop CS2 Windows"
        );

        assert_eq!(
            m.property("http://ns.adobe.com/photoshop/1.0/", "ICCProfile")
                .unwrap(),
            "Dell 1905FP Color Profile"
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

mod property {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta};

    #[test]
    fn happy_path() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert_eq!(
            m.property(xmp_ns::XMP, "CreatorTool"),
            Some("Adobe Photoshop CS2 Windows".to_owned())
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

mod set_property {
    use crate::{tests::fixtures::*, XmpErrorType, XmpMeta};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        m.set_property("http://purl.org/dc/terms/", "provenance", "blah")
            .unwrap();

        assert_eq!(
            m.property("http://purl.org/dc/terms/", "provenance")
                .unwrap(),
            "blah"
        );
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let err = m
            .set_property("http://purl.org/dc/terms/", "", "blah")
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadXPath);
        assert_eq!(err.debug_message, "Empty property name");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let err = m
            .set_property("http://purl.org/dc/terms/", "x\0x", "blah")
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod does_property_exist {
    use crate::{tests::fixtures::*, xmp_ns, XmpMeta};

    #[test]
    fn exists() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(m.does_property_exist(xmp_ns::XMP, "CreatorTool"));
    }

    #[test]
    fn doesnt_exist() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.does_property_exist(xmp_ns::XMP, "RandomProperty"));
    }

    #[test]
    fn empty_namespace() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.does_property_exist("", "CreatorTool"));
    }

    #[test]
    fn empty_prop_name() {
        let m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        assert!(!m.does_property_exist(xmp_ns::XMP, ""));
    }
}

mod set_property_date {
    use crate::{tests::fixtures::*, xmp_ns, XmpDateTime, XmpErrorType, XmpMeta};

    #[test]
    fn happy_path() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime::current().unwrap();

        m.set_property_date(xmp_ns::XMP, "MetadataDate", &updated_time)
            .unwrap();

        // TODO: Read date back when we can.
    }

    #[test]
    fn error_empty_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime::current().unwrap();

        let err = m
            .set_property_date("", "MetadataDate", &updated_time)
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadSchema);
        assert_eq!(err.debug_message, "Empty schema namespace URI");
    }

    #[test]
    fn error_nul_in_name() {
        let mut m = XmpMeta::from_file(fixture_path("Purple Square.psd")).unwrap();
        let updated_time = XmpDateTime::current().unwrap();

        let err = m
            .set_property_date("x\0x", "MetadataDate", &updated_time)
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NulInRustString);
        assert_eq!(
            err.debug_message,
            "Unable to convert to C string because a NUL byte was found"
        );
    }
}

mod array_property {
    use std::str::FromStr;

    use crate::{tests::fixtures::*, XmpMeta, XmpValue};

    #[test]
    fn happy_path_creator_seq() {
        let m = XmpMeta::from_str(PURPLE_SQUARE_XMP).unwrap();

        let mut creators: Vec<XmpValue<String>> = m
            .array_property("http://purl.org/dc/elements/1.1/", "creator")
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
            .array_property("http://purl.org/dc/elements/1.1/", "subject")
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
            .array_property("http://purl.org/dc/elements/1.1/", "creatorx")
            .next();

        assert!(first_creator.is_none());
    }
}
