// Copyright 2023 Adobe. All rights reserved.
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

#[test]
fn exif_latitude_to_decimal() {
    use crate::xmp_gps::exif_latitude_to_decimal as ltd;

    assert_eq!(ltd("47,0N"), Some(47.0));
    assert_eq!(ltd("47,0S"), Some(-47.0));
    assert_eq!(ltd("48,6.750N"), Some(48.1125));

    assert_eq!(ltd(""), None);
    assert_eq!(ltd("47N"), None);
    assert_eq!(ltd("47,0E"), None);
    assert_eq!(ltd("47,4.580"), None);
    assert_eq!(ltd("47,4.580NN"), None);
    assert_eq!(ltd("48,6,750N"), None);

    assert_eq!(ltd("4x7,4.580N"), None);
    assert_eq!(ltd("47,4.58x0N"), None);
}

#[test]
fn exif_longitude_to_decimal() {
    use crate::xmp_gps::exif_longitude_to_decimal as ltd;

    assert_eq!(ltd("47,0E"), Some(47.0));
    assert_eq!(ltd("47,0W"), Some(-47.0));
    assert_eq!(ltd("48,6.750E"), Some(48.1125));

    assert_eq!(ltd(""), None);
    assert_eq!(ltd("47E"), None);
    assert_eq!(ltd("47,0N"), None);
    assert_eq!(ltd("47,4.580"), None);
    assert_eq!(ltd("47,4.580EE"), None);
    assert_eq!(ltd("48,6,750E"), None);

    assert_eq!(ltd("4x7,4.580E"), None);
    assert_eq!(ltd("47,4.58x0E"), None);
}
