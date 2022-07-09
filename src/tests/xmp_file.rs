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

use std::path::PathBuf;

use tempfile::tempdir;

use crate::{tests::fixtures::*, xmp_ns, OpenFileOptions, XmpDateTime, XmpFile, XmpMeta};

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

        let opt_m = f.xmp();
        assert!(opt_m.is_some());

        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms");

        let mut m = opt_m.unwrap();
        m.set_property("http://purl.org/dc/terms/", "provenance", "blah")
            .unwrap();

        assert!(m.does_property_exist("http://purl.org/dc/terms/", "provenance"));
        assert!(!m.does_property_exist("http://purl.org/dc/terms/", "provenancx"));

        if m.does_property_exist(xmp_ns::XMP, "MetadataDate") {
            let updated_time = XmpDateTime::current();
            m.set_property_date(xmp_ns::XMP, "MetadataDate", &updated_time);
        }

        assert!(f.can_put_xmp(&m));
        f.put_xmp(&m);

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
            "blah"
        );
        assert_eq!(m.property("http://purl.org/dc/terms/", "provenancx"), None);
    }
}

#[test]
fn open_fail() {
    let bad_path = PathBuf::from("doesnotexist.jpg");

    {
        let mut f = XmpFile::new().unwrap();

        assert!(f
            .open_file(
                &bad_path,
                OpenFileOptions::default().for_update().use_smart_handler()
            )
            .is_err());
    }
}
