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

mod from_primitive {
    use crate::XmpErrorType;

    #[test]
    fn valid_values() {
        // Sample a few valid i32 to XmpErrorType conversions.
        let et = XmpErrorType::from(0i32);
        assert_eq!(et, XmpErrorType::Unknown);

        let et = XmpErrorType::from(12i32);
        assert_eq!(et, XmpErrorType::UserAbort);

        let et = XmpErrorType::from(213i32);
        assert_eq!(et, XmpErrorType::BadPng);
    }

    #[test]
    fn invalid_values() {
        // Should handle an unexpected value and silently replace with "Unknown."
        let et = XmpErrorType::from(-1i32);
        assert_eq!(et, XmpErrorType::Unknown);

        let et = XmpErrorType::from(214i32);
        assert_eq!(et, XmpErrorType::Unknown);

        let et = XmpErrorType::from(9000i32);
        assert_eq!(et, XmpErrorType::Unknown);
    }
}

mod impl_debug {
    use crate::XmpErrorType;

    #[test]
    fn debug_fmt() {
        assert_eq!(format!("{:#?}", XmpErrorType::Unknown), "Unknown");
        assert_eq!(format!("{:#?}", XmpErrorType::UserAbort), "UserAbort");
        assert_eq!(format!("{:#?}", XmpErrorType::BadPng), "BadPng");
    }
}

mod impl_error {
    use crate::XmpErrorType;

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", XmpErrorType::Unknown), "Unknown error");
        assert_eq!(format!("{}", XmpErrorType::UserAbort), "User abort");
        assert_eq!(format!("{}", XmpErrorType::BadPng), "PNG format error");
    }
}
