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

//! Contains utility functions for mapping XMP/Exif-formatted GPS coordinates
//! to decimal values.

/// Converts an `exif:GPSLatitude` value from XMP/Exif format
/// to the corresponding decimal latitude.
///
/// Will return `None` if the value can not be parsed.
///
/// # Example
/// ```
/// # use xmp_toolkit::xmp_gps;
/// assert_eq!(xmp_gps::exif_latitude_to_decimal("47,0N"), Some(47.0));
/// ```
pub fn exif_latitude_to_decimal(lat: &str) -> Option<f64> {
    if lat.is_empty() {
        return None;
    }

    let (lat, sign_str) = lat.split_at(lat.len() - 1);
    let sign = match sign_str {
        "N" => 1.0,
        "S" => -1.0,
        _ => {
            return None;
        }
    };

    if let Some((deg, min)) = lat.split_once(',') {
        if let Ok(deg) = deg.parse::<f64>() {
            if let Ok(min) = min.parse::<f64>() {
                return Some((deg + (min / 60.0)) * sign);
            }
        }
    }

    None
}

/// Converts an `exif:GPSLongitude` value from XMP/Exif format
/// to the corresponding decimal longitude.
///
/// Will return `None` if the value can not be parsed.
///
/// # Example
/// ```
/// # use xmp_toolkit::xmp_gps;
/// assert_eq!(xmp_gps::exif_longitude_to_decimal("47,0W"), Some(-47.0));
/// ```
pub fn exif_longitude_to_decimal(lat: &str) -> Option<f64> {
    if lat.is_empty() {
        return None;
    }

    let (lat, sign_str) = lat.split_at(lat.len() - 1);
    let sign = match sign_str {
        "E" => 1.0,
        "W" => -1.0,
        _ => {
            return None;
        }
    };

    if let Some((deg, min)) = lat.split_once(',') {
        if let Ok(deg) = deg.parse::<f64>() {
            if let Ok(min) = min.parse::<f64>() {
                return Some((deg + (min / 60.0)) * sign);
            }
        }
    }

    None
}
