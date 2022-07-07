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

/// Describes error conditions returned by most XMP Toolkit operations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub struct XmpError {
    /// A selector for the specific error type.
    pub error_type: XmpErrorType,

	/// Descriptive string, for debugging use only. It must not be shown to users in a final
	/// product. It is written for developers, not users, and never localized.
    pub debug_description: String,
}

/// Describes which error type occurred.
#[non_exhaustive]
pub enum XmpErrorType {
    // --- Generic error codes ---

	/// Generic unknown error.
    Unknown = 0,

    /// Generic undefined error.
    Tbd = 1,

    /// Generic unavailable error.
    Unavailable = 2,

    /// Generic bad object error.
    BadObject = 3,

    /// Generic bad parameter error.
    BadParam = 4,

    /// Generic bad value error.
    BadValue = 5,

    /// Generic assertion failure.
    AssertFailure = 6,

    /// Generic enforcement failure.
    EnforceFailure = 7,

    /// Generic unimplemented error.
    Unimplemented = 8,

    /// Generic internal failure.
    InternalFailure = 9,

    /// Generic deprecated error.
    Deprecated = 10,

    /// Generic external failure.
    ExternalFailure = 11,

    /// Generic user abort error.
    UserAbort = 12,

    /// Generic standard exception.
    StdException = 13,

    /// Generic unknown exception.
    UnknownException = 14,

    /// Generic out-of-memory error.
    NoMemory = 15,

    /// Progress reporting callback requested abort.
    ProgressAbort = 16,

    // --- More specific parameter error codes ---

	/// Bad schema parameter.
    BadSchema = 101,

    /// Bad XPath parameter.
    BadXPath = 102,

	/// Bad options parameter.
    BadOptions = 103,

    /// Bad index parameter.
    BadIndex = 104,

    /// Bad iteration position.
    BadIterPosition = 105,

    /// XML parsing error (deprecated).
    BadParse = 106,

    /// Serialization error.
    BadSerialize = 107,

    /// File format error.
    BadFileFormat = 108,

    /// No file handler found for format.
    NoFileHandler = 109,

    /// Data too large for JPEG file format.
    TooLargeForJpeg = 110,

    /// A file does not exist.
    NoFile = 111,

    /// A file exists but cannot be opened.
    FilePermission = 112,

    /// A file write failed due to lack of disk space.
    DiskSpace = 113,

    /// A file read failed.
    ReadError = 114,

    /// A file write failed for a reason other than lack of disk space.
    WriteError = 115,

    /// A block of a file is ill-formed, e.g. invalid IPTC-IIM in a photo.
    BadBlockFormat = 116,

    /// File path is not a file.
    FilePathNotAFile = 117,

	/// Rejected file extension.
	RejectedFileExtension = 118,

    // --- File format and internal structure error codes ---

	/// XML format error.
    BadXml = 201,

    /// RDF format error.
    BadRdf = 202,

    /// XMP format error.
    BadXmp = 203,

    /// Empty iterator.
    EmptyIterator = 204,

    /// Unicode error.
    BadUnicode = 205,

    /// TIFF format error.
    BadTiff = 206,

    /// JPEG format error.
    BadJpeg = 207,

	/// PSD format error.
    BadPsd = 208,

    /// PSIR format error.
    BadPsir = 209,

    /// IPTC format error.
    BadIptc = 210,

    /// MPEG format error.
    BadMpeg = 211,

    /// HEIF format: Modify Operation is not supported for Construction Method 1 or 2.
	HeifConstructionMethodNotSupported = 212,

    /// PNG format error.
	BadPng = 213,
}

/// A specialized `Result` type for XMP Toolkit operations.
pub type Result<T> = std::result::Result<T, XmpError>;
