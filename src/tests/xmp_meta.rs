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

use crate::{tests::fixtures::*, XmpMeta};

#[test]
fn new_empty() {
    let mut _m = XmpMeta::new();
}

#[test]
fn from_file() {
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
fn register_namespace() {
    assert_eq!(
        XmpMeta::register_namespace("http://purl.org/dc/terms/", "dcterms"),
        "dcterms:"
    );
}
