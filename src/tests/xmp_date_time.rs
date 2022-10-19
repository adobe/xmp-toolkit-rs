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

use crate::XmpDateTime;

#[test]
fn default() {
    let dt = XmpDateTime::default();
    assert!(dt.date.is_none());
    assert!(dt.time.is_none());
}

#[test]
fn current() {
    let dt = XmpDateTime::current().unwrap();

    let date = dt.date.as_ref().unwrap();
    assert!(date.year >= 2022);
    assert!(date.month >= 1);
    assert!(date.month <= 12);
    assert!(date.day >= 1);
    assert!(date.day <= 31);
}
