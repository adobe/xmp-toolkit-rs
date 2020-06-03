// TO DO: Revise API documentation to fit the Rust wrapper.

use bitflags::bitflags;
use std::ffi::CString;

use crate::ffi;
use crate::xmp_meta::XmpMeta;
use crate::xmp_toolkit::XmpToolkit;

bitflags! {
    /// Option flags for \c XMPFile::open_file().
    /// Flags describing the set of lua modules to load.
    pub struct OpenFileOptions: u32 {
        /// Open for read-only access.
        const OPEN_FOR_READ = 0x00000001;

        /// Open for reading and writing.
        const OPEN_FOR_UPDATE = 0x00000002;

        /// Only the XMP is wanted, allows space/time optimizations.
        const OPEN_ONLY_XMP = 0x00000004;

        /// Force use of the given handler (format), do not even verify the format.
        const FORCE_GIVEN_HANDLER = 0x00000008;

        /// Be strict about only attempting to use the designated file handler,
        /// no fallback to other handlers.
        const OPEN_STRICTLY = 0x00000010;

        /// Require the use of a smart handler.
        const OPEN_USE_SMART_HANDLER = 0x00000020;

        /// Force packet scanning, do not use a smart handler.
        const OPEN_USE_PACKET_SCANNING = 0x00000040;

        /// Only packet scan files "known" to need scanning.
        const OPEN_LIMITED_SCANNING = 0x00000080;

        /// Attempt to repair a file opened for update, default is to not open (throw an exception).
        const OPEN_REPAIR_FILE = 0x00000100;

        /// When updating a file, spend the effort necessary to optimize file layout.
        const OPTIMIZE_FILE_LAYOUT = 0x00000200;
    }
}

#[allow(dead_code)] // because xmp is never used
pub struct XmpFile<'xmp> {
    f: *mut ffi::CXmpFile,
    xmp: &'xmp XmpToolkit,
}

/// XMP File result codes
#[derive(Debug)]
pub enum XmpFileError {
    CantOpenFile,
}

impl<'xmp> Drop for XmpFile<'xmp> {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpFileDrop(self.f);
        }
    }
}

impl<'xmp> XmpFile<'xmp> {
    /// Creates a new file struct that is associated with no file.
    pub fn new(xmp: &'xmp XmpToolkit) -> XmpFile<'xmp> {
        let f = unsafe { ffi::CXmpFileNew() };
        XmpFile { f, xmp }
    }

    /// Opens a file for the requested forms of metadata access. Opening the file at a minimum
    /// causes the raw XMP packet to be read from the file. If the file handler supports legacy
    /// metadata reconciliation then legacy metadata is also read, unless \c #kXMPFiles_OpenOnlyXMP
    /// is passed.
    ///
    /// If the file is opened for read-only access (passing \c #kXMPFiles_OpenForRead), the disk
    /// file is closed immediately after reading the data from it; the \c XMPFiles object, however,
    /// remains in the open state. You must call \c CloseFile() when finished using it. Other
    /// methods, such as \c GetXMP(), can only be used between the \c OpenFile() and \c CloseFile()
    /// calls. The \c XMPFiles destructor does not call \c CloseFile(); if you call it without
    /// closing, any pending updates are lost.
    ///
    /// If the file is opened for update (passing \c #kXMPFiles_OpenForUpdate), the disk file
    /// remains open until \c CloseFile() is called. The disk file is only updated once, when
    /// \c CloseFile() is called, regardless of how many calls are made to \c PutXMP().
    ///
    /// Typically, the XMP is not parsed and legacy reconciliation is not performed until \c GetXMP()
    /// is called, but this is not guaranteed. Specific file handlers might do earlier parsing of
    /// the XMP. Delayed parsing and early disk file close for read-only access are optimizations
    /// to help clients implementing file browsers, so that they can access the file briefly
    /// and possibly display a thumbnail, then postpone more expensive XMP processing until later.
    ///
    /// @param path The path for the file.
    ///
    /// @param flags A set of option flags that describe the desired access. By default (zero)
    /// the file is opened for read-only access and the format handler decides on the level of
    /// reconciliation that will be performed. A logical OR of these bit-flag constants:
    ///
    ///   \li \c #kXMPFiles_OpenForRead - Open for read-only access.
    ///   \li \c #kXMPFiles_OpenForUpdate - Open for reading and writing.
    ///   \li \c #kXMPFiles_OpenOnlyXMP - Only the XMP is wanted, no reconciliation.
    ///   \li \c #kXMPFiles_OpenStrictly - Be strict about locating XMP and reconciling with other
    ///   forms. By default, a best effort is made to locate the correct XMP and to reconcile XMP
    ///   with other forms (if reconciliation is done). This option forces stricter rules, resulting
    ///   in exceptions for errors. The definition of strictness is specific to each handler, there
    ///   might be no difference.
    ///   \li \c #kXMPFiles_OpenUseSmartHandler - Require the use of a smart handler.
    ///   \li \c #kXMPFiles_OpenUsePacketScanning - Force packet scanning, do not use a smart handler.
    ///   \li \c #kXMPFiles_OptimizeFileLayout - When updating a file, spend the effort necessary
    ///    to optimize file layout.
    pub fn open_file(&mut self, path: &str, flags: OpenFileOptions) -> Result<(), XmpFileError> {
        let c_path = CString::new(path).unwrap();
        let ok = unsafe { ffi::CXmpFileOpen(self.f, c_path.as_ptr(), flags.bits()) };

        if ok != 0 {
            Ok(())
        } else {
            Err(XmpFileError::CantOpenFile)
        }
    }

    /// Retrieves the XMP metadata from an open file.
    ///
    /// If no XMP is present, will return `None`.
    pub fn get_xmp(&mut self) -> Option<XmpMeta<'xmp>> {
        unsafe {
            let m = ffi::CXmpFileGetXMP(self.f);
            if m.is_null() {
                None
            } else {
                Some(XmpMeta { m, xmp: self.xmp })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use crate::XmpToolkit;

    use super::*;

    fn fixture_path(name: &str) -> String {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut path = PathBuf::from(root_dir);
        path.push("tests/fixtures");
        path.push(name);
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn open_file() {
        let purple_square = fixture_path("Purple Square.psd");

        let xmp = XmpToolkit::new();
        let mut f = XmpFile::new(&xmp);

        assert!(f
            .open_file(&purple_square, OpenFileOptions::OPEN_FOR_READ)
            .is_ok());

        let opt_m = f.get_xmp();
        assert!(opt_m.is_some());
    }
}
