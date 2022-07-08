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

use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) fn fixture_path(name: &str) -> String {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut path = PathBuf::from(root_dir);
    path.push("src/tests/fixtures");
    path.push(name);

    assert!(path.exists());

    path.to_str().unwrap().to_string()
}

pub(crate) fn temp_copy_of_fixture(tempdir: &Path, name: &str) -> String {
    let fixture_src = fixture_path(name);
    let fixture_path = Path::join(tempdir, name);
    let fixture_copy = fixture_path.as_path();

    fs::copy(fixture_src, fixture_copy).unwrap();
    fixture_copy.display().to_string()
}
