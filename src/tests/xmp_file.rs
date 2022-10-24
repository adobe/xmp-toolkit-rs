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

use tempfile::tempdir;

use crate::{tests::fixtures::*, xmp_ns, OpenFileOptions, XmpDateTime, XmpFile, XmpMeta, XmpValue};

#[test]
fn open_and_edit_file() {
    let tempdir = tempdir().unwrap();
    let purple_square = temp_copy_of_fixture(tempdir.path(), "Purple Square.psd");

    {
        let mut f = XmpFile::new().unwrap();

        assert!(f
            .open_file(
                &purple_square,
                OpenFileOptions::default().for_update().use_smart_handler()
            )
            .is_ok());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let mut m = f.xmp().unwrap();
        m.set_property("http://purl.org/dc/terms/", "provenance", &"blah".into())
            .unwrap();

        assert!(m.contains_property("http://purl.org/dc/terms/", "provenance"));
        assert!(!m.contains_property("http://purl.org/dc/terms/", "provenancx"));

        if m.contains_property(xmp_ns::XMP, "MetadataDate") {
            let updated_time = XmpDateTime::current().unwrap();
            m.set_property_date(xmp_ns::XMP, "MetadataDate", &updated_time.into())
                .unwrap();
        }

        assert!(f.can_put_xmp(&m));
        f.put_xmp(&m).unwrap();

        f.close();
    }

    // Let's make sure we actually wrote to the file.
    {
        let mut f = XmpFile::new().unwrap();

        assert!(f
            .open_file(
                &purple_square,
                OpenFileOptions::default().for_update().use_smart_handler()
            )
            .is_ok());

        let m = f.xmp().unwrap();

        assert_eq!(
            m.property("http://purl.org/dc/terms/", "provenance")
                .unwrap(),
            XmpValue {
                value: "blah".to_owned(),
                options: 0
            }
        );
        assert_eq!(m.property("http://purl.org/dc/terms/", "provenancx"), None);
    }
}

mod open_file {
    use std::path::PathBuf;

    use crate::{OpenFileOptions, XmpErrorType, XmpFile};

    #[test]
    fn file_not_found() {
        let mut f = XmpFile::new().unwrap();
        let bad_path = PathBuf::from("doesnotexist.jpg");

        let err = f
            .open_file(&bad_path, OpenFileOptions::default())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NoFile);
    }

    #[test]
    fn cant_convert_path() {
        let mut f = XmpFile::new().unwrap();
        let bad_path = PathBuf::from("doesn\0texist.jpg");

        let err = f
            .open_file(&bad_path, OpenFileOptions::default())
            .unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::BadParam);
        assert_eq!(err.debug_message, "Could not convert path to C string");
    }
}

mod get_xmp {
    use crate::{tests::fixtures::*, OpenFileOptions, XmpFile};

    #[test]
    fn no_xmp_in_file() {
        let mut f = XmpFile::new().unwrap();

        let no_xmp = fixture_path("no_xmp.txt");
        assert!(f.open_file(&no_xmp, OpenFileOptions::default()).is_ok());

        let opt_m = f.xmp();
        assert!(opt_m.is_none());
    }
}

mod can_put_xmp {
    use tempfile::tempdir;

    use crate::{tests::fixtures::*, OpenFileOptions, XmpFile, XmpMeta};

    #[test]
    fn no_xmp_in_file() {
        let tempdir = tempdir().unwrap();
        let no_xmp = temp_copy_of_fixture(tempdir.path(), "no_xmp.txt");

        let mut f = XmpFile::new().unwrap();
        assert!(f
            .open_file(&no_xmp, OpenFileOptions::default().for_update())
            .is_ok());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let mut m = XmpMeta::new().unwrap();
        m.set_property("http://purl.org/dc/terms/", "provenance", &"blah".into())
            .unwrap();

        assert!(!f.can_put_xmp(&m));
    }

    #[test]
    fn init_fail() {
        let tempdir = tempdir().unwrap();
        let no_xmp = temp_copy_of_fixture(tempdir.path(), "no_xmp.txt");

        let mut f = XmpFile::new().unwrap();
        assert!(f
            .open_file(&no_xmp, OpenFileOptions::default().for_update())
            .is_ok());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let m = XmpMeta::new_fail();
        assert!(!f.can_put_xmp(&m));
    }
}

mod put_xmp {
    use tempfile::tempdir;

    use crate::{tests::fixtures::*, OpenFileOptions, XmpErrorType, XmpFile, XmpMeta};

    #[test]
    fn no_xmp_in_file() {
        let tempdir = tempdir().unwrap();
        let no_xmp = temp_copy_of_fixture(tempdir.path(), "no_xmp.txt");

        let mut f = XmpFile::new().unwrap();
        assert!(f
            .open_file(&no_xmp, OpenFileOptions::default().for_update())
            .is_ok());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let mut m = XmpMeta::new().unwrap();
        m.set_property("http://purl.org/dc/terms/", "provenance", &"blah".into())
            .unwrap();

        let err = f.put_xmp(&m).unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::Unavailable);
        assert_eq!(err.debug_message, "XMPFiles::PutXMP - Can't inject XMP");
    }

    #[test]
    fn init_fail() {
        let tempdir = tempdir().unwrap();
        let no_xmp = temp_copy_of_fixture(tempdir.path(), "no_xmp.txt");

        let mut f = XmpFile::new().unwrap();
        assert!(f
            .open_file(&no_xmp, OpenFileOptions::default().for_update())
            .is_ok());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms").unwrap();

        let m = XmpMeta::new_fail();
        let err = f.put_xmp(&m).unwrap_err();

        assert_eq!(err.error_type, XmpErrorType::NoCppToolkit);
    }
}
