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

mod raise_from_c {
    use std::{ffi::CString, ptr};

    use crate::{ffi::CXmpError, XmpError, XmpErrorType};

    #[test]
    fn no_error() {
        let c = CXmpError {
            had_error: 0,
            id: 0,
            debug_message: ptr::null(),
        };

        let result = XmpError::raise_from_c(&c);
        assert!(result.is_ok());
    }

    #[test]
    fn unknown_error_without_debug_message() {
        let c = CXmpError {
            had_error: 1,
            id: 0,
            debug_message: ptr::null(),
        };

        let err = XmpError::raise_from_c(&c).unwrap_err();
        assert_eq!(err.error_type, XmpErrorType::Unknown);
        assert_eq!(err.debug_message, "");
    }

    #[test]
    fn unknown_error_with_debug_message() {
        let msg = CString::new("sample message").unwrap();

        let c = CXmpError {
            had_error: 1,
            id: 0,
            debug_message: msg.as_ptr(),
        };

        let err = XmpError::raise_from_c(&c).unwrap_err();
        assert_eq!(err.error_type, XmpErrorType::Unknown);
        assert_eq!(err.debug_message, "sample message");
    }

    #[test]
    fn user_abort_error() {
        let c = CXmpError {
            had_error: 1,
            id: 12,
            debug_message: ptr::null(),
        };

        let err = XmpError::raise_from_c(&c).unwrap_err();
        assert_eq!(err.error_type, XmpErrorType::UserAbort);
        assert_eq!(err.debug_message, "");
    }

    #[test]
    fn bad_id() {
        let msg = CString::new("bogus XMP error").unwrap();

        let c = CXmpError {
            had_error: 1,
            id: 9000,
            debug_message: msg.as_ptr(),
        };

        let err = XmpError::raise_from_c(&c).unwrap_err();
        assert_eq!(err.error_type, XmpErrorType::Unknown);
        assert_eq!(err.debug_message, "bogus XMP error");
    }
}

mod impl_debug {
    use crate::{XmpError, XmpErrorType};

    #[test]
    fn without_debug_message() {
        let err = XmpError {
            error_type: XmpErrorType::BadJpeg,
            debug_message: "".to_owned(),
        };

        assert_eq!(
            format!("{:#?}", err),
            "XmpError {\n    error_type: BadJpeg,\n    debug_message: \"\",\n}"
        );
    }

    #[test]
    fn with_debug_message() {
        let err = XmpError {
            error_type: XmpErrorType::NoFile,
            debug_message: "sample message".to_owned(),
        };

        assert_eq!(
            format!("{:#?}", err),
            "XmpError {\n    error_type: NoFile,\n    debug_message: \"sample message\",\n}"
        );
    }
}

mod impl_error {
    use crate::{XmpError, XmpErrorType};

    #[test]
    fn without_debug_message() {
        let err = XmpError {
            error_type: XmpErrorType::BadJpeg,
            debug_message: "".to_owned(),
        };

        assert_eq!(format!("{}", err), "XmpError(JPEG format error)");
    }

    #[test]
    fn with_debug_message() {
        let err = XmpError {
            error_type: XmpErrorType::NoFile,
            debug_message: "sample message".to_owned(),
        };

        assert_eq!(
            format!("{}", err),
            "XmpError(File not found, sample message)"
        );
    }
}
