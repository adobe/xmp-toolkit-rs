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

use std::{ffi::CString, fmt, path::Path};

use crate::{ffi, xmp_meta::XmpMeta};

/// The `XmpFile` struct allows access to the main (document-level) metadata in a file.
///
/// This provides convenient access to the main, or document level, XMP for a file. Use
/// it to obtain metadata from a file, which you can then manipulate with the [`XmpMeta`]
/// struct; and to write new or changed metadata back out to a file.
///
/// The functions allow you to open a file, read and write the metadata, then close the file.
/// While open, portions of the file might be maintained in RAM data structures. Memory
/// usage can vary considerably depending on file format and access options.
///
/// A file can be opened for read-only or read-write access, with typical exclusion for both
/// modes.
pub struct XmpFile {
    f: *mut ffi::CXmpFile,
}

impl Drop for XmpFile {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpFileDrop(self.f);
        }
    }
}

impl Default for XmpFile {
    fn default() -> Self {
        XmpFile::new()
    }
}

impl XmpFile {
    /// Creates a new file struct that is associated with no file.
    pub fn new() -> XmpFile {
        XmpFile {
            f: unsafe { ffi::CXmpFileNew() },
        }
    }

    /// Opens a file for the requested forms of metadata access.
    ///
    /// Opening the file, at a minimum, causes the raw XMP packet to be read from the file.
    /// If the file handler supports legacy metadata reconciliation then legacy metadata is
    /// also read, unless `kXMPFiles_OpenOnlyXMP` is passed.
    ///
    /// If the file is opened for read-only access (passing [`OpenFileOptions::for_read()`]),
    /// the disk file is closed immediately after reading the data from it; the `XMPFile` struct,
    /// however, remains in the open state until `drop()` is called.
    ///
    /// If you update the XMP, you must call [`XmpFile::put_xmp()`] before the struct is dropped;
    /// if you do not, any pending updates are lost.
    ///
    /// Typically, the XMP is not parsed and legacy reconciliation is not performed until
    /// [`XmpFile::xmp()`] is called, but this is not guaranteed. Specific file handlers might
    /// do earlier parsing of the XMP. Delayed parsing and early disk file close for read-only
    /// access are optimizations to help clients implementing file browsers, so that they can
    /// access the file briefly and possibly display a thumbnail, then postpone more expensive
    /// XMP processing until later.
    ///
    /// ## Arguments
    ///
    /// * `path`: The path for the file.
    ///
    /// * `flags`: A set of option flags that describe the desired access. By default
    /// ([`OpenFileOptions::default()`]), the file is opened for read-only access and the
    /// format handler decides on the level of reconciliation that will be performed.
    /// See [`OpenFileOptions`] for other options.
    pub fn open_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        flags: OpenFileOptions,
    ) -> Result<(), XmpFileError> {
        if let Some(c_path) = path_to_cstr(path.as_ref()) {
            if unsafe { ffi::CXmpFileOpen(self.f, c_path.as_ptr(), flags.options) } != 0 {
                return Ok(());
            }
        }

        Err(XmpFileError::CantOpenFile)
    }

    /// Retrieves the XMP metadata from an open file.
    ///
    /// If no XMP is present, will return `None`.
    pub fn xmp(&mut self) -> Option<XmpMeta> {
        unsafe {
            let m = ffi::CXmpFileGetXmp(self.f);
            if m.is_null() {
                None
            } else {
                Some(XmpMeta { m })
            }
        }
    }

    /// Reports whether this file can be updated with a specific XMP packet.
    ///
    /// Use this function to determine if the file can probably be updated with a
    /// given set of XMP metadata. This depends on the size of the packet, the
    /// options with which the file was opened, and the capabilities of the handler
    /// for the file format. The function obtains the length of the serialized
    /// packet for the provided XMP, but does not keep it or modify it, and does not
    /// cause the file to be written when closed.
    pub fn can_put_xmp(&self, meta: &XmpMeta) -> bool {
        let r = unsafe { ffi::CXmpFileCanPutXmp(self.f, meta.m) };
        r != 0
    }

    /// Updates the XMP metadata in this object without writing out the file.
    ///
    /// This function supplies new XMP for the file. However, the disk file is not written until
    /// the struct is closed with [`XmpFile::close()`]. The options provided when the file was opened
    /// determine if reconciliation is done with other forms of metadata.
    pub fn put_xmp(&mut self, meta: &XmpMeta) {
        unsafe { ffi::CXmpFilePutXmp(self.f, meta.m) };
    }

    /// Explicitly closes an opened file.
    ///
    /// Performs any necessary output to the file and closes it. Files that are opened
    /// for update are written to only when closing.
    ///
    /// If the file is opened for read-only access (passing [`OpenFileOptions::for_read()`]),
    /// the disk file is closed immediately after reading the data from it; the `XMPFile`
    /// struct, however, remains in the open state. You must call [`XmpFile::close()`] when finished
    /// using it. Other methods, such as [`XmpFile::xmp()`], can only be used between the
    /// [`XmpFile::open_file()`] and [`XmpFile::close()`] calls. The `XMPFile` destructor does
    /// not call [`XmpFile::close()`]; if the struct is dropped without closing, any pending
    /// updates are lost.
    ///
    /// If the file is opened for update (passing [`OpenFileOptions::for_update()`]),
    /// the disk file remains open until [`XmpFile::close()`] is called. The disk file is only
    /// updated once, when [`XmpFile::close()`] is called, regardless of how many calls are
    /// made to [`XmpFile::put_xmp()`].
    pub fn close(&mut self) {
        unsafe { ffi::CXmpFileClose(self.f) };
    }
}

/// Option flags for [`XmpFile::open_file()`].
///
/// Invoke by calling [`OpenFileOptions::default()`] and then calling methods
/// on this struct to add options as needed.
#[derive(Default)]
pub struct OpenFileOptions {
    pub(crate) options: u32,
}

impl OpenFileOptions {
    /// Open for read-only access.
    ///
    /// See `kXMPFiles_OpenForRead` constant in C++ SDK.
    pub fn for_read(mut self) -> Self {
        self.options |= 0x00000001;
        self
    }

    /// Open for reading and writing.
    ///
    /// See `kXMPFiles_OpenForUpdate` constant in C++ SDK.
    pub fn for_update(mut self) -> Self {
        self.options |= 0x00000002;
        self
    }

    /// Only the XMP is wanted.
    ///
    /// This allows space/time optimizations.
    ///
    /// See `kXMPFiles_OpenOnlyXMP` constant in C++ SDK.
    pub fn only_xmp(mut self) -> Self {
        self.options |= 0x00000004;
        self
    }

    /// Force use of the given handler (format).
    ///
    /// Do not even verify the format.
    ///
    /// See `kXMPFiles_ForceGivenHandler` constant in C++ SDK.
    pub fn force_given_handler(mut self) -> Self {
        self.options |= 0x00000008;
        self
    }

    /// Be strict about only attempting to use the designated file handler.
    ///
    /// Do not fall back to other handlers.
    ///
    /// See `kXMPFiles_OpenStrictly` constant in C++ SDK.
    pub fn strict(mut self) -> Self {
        self.options |= 0x00000010;
        self
    }

    /// Require the use of a smart handler.
    ///
    /// See `kXMPFiles_OpenUseSmartHandler` constant in C++ SDK.
    pub fn use_smart_handler(mut self) -> Self {
        self.options |= 0x00000020;
        self
    }

    /// Force packet scanning.
    ///
    /// Do not use a smart handler.
    ///
    /// See `kXMPFiles_OpenUsePacketScanning` constant in C++ SDK.
    pub fn use_packet_scanning(mut self) -> Self {
        self.options |= 0x00000040;
        self
    }

    /// Only packet scan files "known" to need scanning.
    ///
    /// See `kXMPFiles_OpenLimitedScanning` constant in C++ SDK.
    pub fn limited_scanning(mut self) -> Self {
        self.options |= 0x00000080;
        self
    }

    /// Attempt to repair a file opened for update.
    ///
    /// Default is to not open (throw an exception).
    ///
    /// See `kXMPFiles_OpenRepairFile` constant in C++ SDK.
    pub fn repair_file(mut self) -> Self {
        self.options |= 0x00000100;
        self
    }

    /// When updating a file, spend the effort necessary to optimize file layout.
    ///
    /// See `kXMPFiles_OptimizeFileLayout` constant in C++ SDK.
    pub fn optimize_file_layout(mut self) -> Self {
        self.options |= 0x00000200;
        self
    }
}

/// Describes the potential error conditions that might arise from [`XmpFile`] operations.
#[derive(Debug)]
pub enum XmpFileError {
    /// Returned if the XMP Toolkit could not open the file.
    CantOpenFile,
}

impl fmt::Display for XmpFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::CantOpenFile => {
                write!(f, "could not open XMP content from this file")
            }
        }
    }
}

impl std::error::Error for XmpFileError {}

fn path_to_cstr(path: &Path) -> Option<CString> {
    match path.to_str() {
        Some(path_str) => match CString::new(path_str) {
            Ok(c_path) => Some(c_path),
            Err(_) => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        env, fs,
        path::{Path, PathBuf},
    };

    use coverage_helper::test;
    use tempfile::tempdir;

    use crate::{xmp_ns, XmpDateTime};

    fn fixture_path(name: &str) -> String {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();

        let mut path = PathBuf::from(root_dir);
        path.push("tests/fixtures");
        path.push(name);
        path.to_str().unwrap().to_string()
    }

    fn temp_copy_of_fixture(tempdir: &Path, name: &str) -> String {
        let fixture_src = fixture_path(name);
        let fixture_path = Path::join(tempdir, name);
        let fixture_copy = fixture_path.as_path();

        fs::copy(fixture_src, fixture_copy).unwrap();
        fixture_copy.display().to_string()
    }

    #[test]
    fn open_and_edit_file() {
        let tempdir = tempdir().unwrap();
        let purple_square = temp_copy_of_fixture(tempdir.path(), "Purple Square.psd");

        {
            let mut f = XmpFile::new();

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
            m.set_property("http://purl.org/dc/terms/", "provenance", "blah");

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
            let mut f = XmpFile::new();

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
            let mut f = XmpFile::new();

            assert!(f
                .open_file(
                    &bad_path,
                    OpenFileOptions::default().for_update().use_smart_handler()
                )
                .is_err());
        }
    }
}
