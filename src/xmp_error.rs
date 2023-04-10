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
    ffi::{CStr, NulError},
    fmt,
};

use num_enum::FromPrimitive;
use thiserror::Error;

use crate::ffi::CXmpError;

/// Describes error conditions returned by XMP Toolkit operations.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct XmpError {
    /// A selector for the specific error type.
    pub error_type: XmpErrorType,

    /// Descriptive string, for debugging use only. It must not be shown to
    /// users in a final product. It is written for developers, not users,
    /// and never localized.
    pub debug_message: String,
}

impl XmpError {
    pub(crate) fn raise_from_c(err: &CXmpError) -> XmpResult<()> {
        if err.had_error != 0 {
            Err(XmpError {
                error_type: XmpErrorType::from(err.id),
                debug_message: if err.debug_message.is_null() {
                    String::default()
                } else {
                    unsafe {
                        CStr::from_ptr(err.debug_message)
                            .to_string_lossy()
                            .into_owned()
                    }
                },
            })
        } else {
            Ok(())
        }
    }
}

impl From<NulError> for XmpError {
    fn from(_: NulError) -> Self {
        XmpError {
            error_type: XmpErrorType::NulInRustString,
            debug_message: "Unable to convert to C string because a NUL byte was found".to_owned(),
        }
    }
}

impl fmt::Display for XmpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.debug_message.is_empty() {
            write!(f, "XmpError({})", self.error_type)
        } else {
            write!(f, "XmpError({}, {})", self.error_type, self.debug_message)
        }
    }
}

impl std::error::Error for XmpError {}

/// Describes which error type occurred.
///
/// Represents a specific error code from the underlying C++ XMP Toolkit.
#[derive(Debug, Eq, Error, FromPrimitive, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum XmpErrorType {
    // --- Generic error codes ---
    /// Generic unknown error.
    #[error("Unknown error")]
    #[default]
    Unknown = 0,

    /// Generic undefined error.
    #[error("Undefined error")]
    Tbd = 1,

    /// Generic unavailable error.
    #[error("Unavailable error")]
    Unavailable = 2,

    /// Generic bad object error.
    #[error("Bad object")]
    BadObject = 3,

    /// Generic bad parameter error.
    #[error("Bad parameter")]
    BadParam = 4,

    /// Generic bad value error.
    #[error("Bad value")]
    BadValue = 5,

    /// Generic assertion failure.
    #[error("Assertion failure")]
    AssertFailure = 6,

    /// Generic enforcement failure.
    #[error("Enforcement failure")]
    EnforceFailure = 7,

    /// Generic unimplemented error.
    #[error("Unimplemented")]
    Unimplemented = 8,

    /// Generic internal failure.
    #[error("Internal failure")]
    InternalFailure = 9,

    /// Generic deprecated error.
    #[error("Deprecated")]
    Deprecated = 10,

    /// Generic external failure.
    #[error("External failure")]
    ExternalFailure = 11,

    /// Generic user abort error.
    #[error("User abort")]
    UserAbort = 12,

    /// Generic standard exception.
    #[error("Standard exception")]
    StdException = 13,

    /// Generic unknown exception.
    #[error("Unknown exception")]
    UnknownException = 14,

    /// Generic out-of-memory error.
    #[error("Out of memory")]
    NoMemory = 15,

    /// Progress reporting callback requested abort.
    #[error("Progress callback requested abort")]
    ProgressAbort = 16,

    // --- More specific parameter error codes ---
    /// Bad schema parameter.
    #[error("Bad schema")]
    BadSchema = 101,

    /// Bad XPath parameter.
    #[error("Bad XPath")]
    BadXPath = 102,

    /// Bad options parameter.
    #[error("Bad options parameter")]
    BadOptions = 103,

    /// Bad index parameter.
    #[error("Bad index parameter")]
    BadIndex = 104,

    /// Bad iteration position.
    #[error("Bad iteration position")]
    BadIterPosition = 105,

    /// XML parsing error (deprecated).
    #[error("XML parsing error")]
    BadParse = 106,

    /// Serialization error.
    #[error("Serialization error")]
    BadSerialize = 107,

    /// File format error.
    #[error("File format error")]
    BadFileFormat = 108,

    /// No file handler found for format.
    #[error("No file handler found for format")]
    NoFileHandler = 109,

    /// Data too large for JPEG file format.
    #[error("Data too large for JPEG file format")]
    TooLargeForJpeg = 110,

    /// A file does not exist.
    #[error("File not found")]
    NoFile = 111,

    /// A file exists but cannot be opened.
    #[error("File permission error")]
    FilePermission = 112,

    /// A file write failed due to lack of disk space.
    #[error("Out of disk space")]
    DiskSpace = 113,

    /// A file read failed.
    #[error("Unable to read file")]
    ReadError = 114,

    /// A file write failed for a reason other than lack of disk space.
    #[error("Unable to write file")]
    WriteError = 115,

    /// A block of a file is ill-formed, e.g. invalid IPTC-IIM in a photo.
    #[error("File format error")]
    BadBlockFormat = 116,

    /// File path is not a file.
    #[error("Path does not point to a file")]
    FilePathNotAFile = 117,

    /// Rejected file extension.
    #[error("Rejected file extension")]
    RejectedFileExtension = 118,

    // --- File format and internal structure error codes ---
    /// XML format error.
    #[error("XML format error")]
    BadXml = 201,

    /// RDF format error.
    #[error("RDF format error")]
    BadRdf = 202,

    /// XMP format error.
    #[error("XMP format error")]
    BadXmp = 203,

    /// Empty iterator.
    #[error("Empty iterator")]
    EmptyIterator = 204,

    /// Unicode error.
    #[error("Unicode error")]
    BadUnicode = 205,

    /// TIFF format error.
    #[error("TIFF format error")]
    BadTiff = 206,

    /// JPEG format error.
    #[error("JPEG format error")]
    BadJpeg = 207,

    /// PSD format error.
    #[error("PSD format error")]
    BadPsd = 208,

    /// PSIR format error.
    #[error("PSIR format error")]
    BadPsir = 209,

    /// IPTC format error.
    #[error("IPTC format error")]
    BadIptc = 210,

    /// MPEG format error.
    #[error("MPEG format error")]
    BadMpeg = 211,

    /// HEIF format: Modify Operation is not supported for Construction Method 1
    /// or 2.
    #[error("HEIF construction method not supported")]
    HeifConstructionMethodNotSupported = 212,

    /// PNG format error.
    #[error("PNG format error")]
    BadPng = 213,

    // --- Rust-specific errors ---
    /// Can not convert from Rust string to C string because a NUL byte was
    /// found.
    #[error("Unable to convert to C string because a NUL byte was found")]
    NulInRustString = -432,

    /// C++ toolkit did not initialize properly.
    #[error("C++ XMP toolkit did not initialize properly")]
    NoCppToolkit = -433,

    /// An `x:xmpmeta` wrapper was required, but not found.
    ///
    /// This error can only occur if you call
    /// [`XmpMeta::from_str_with_options()`]
    /// with [`FromStrOptions::require_xmp_meta()`].
    ///
    /// [`XmpMeta::from_str_with_options()`]: crate::XmpMeta::from_str_with_options
    /// [`FromStrOptions::require_xmp_meta()`]: crate::FromStrOptions::require_xmp_meta
    #[error("x:xmpmeta element not found")]
    XmpMetaElementMissing = -434,
}

/// A specialized `Result` type for XMP Toolkit operations.
pub type XmpResult<T> = std::result::Result<T, XmpError>;
