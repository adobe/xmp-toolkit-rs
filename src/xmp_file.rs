use crate::ffi;
use crate::xmp_toolkit::XmpToolkit;

#[allow(dead_code)] // because xmp is never used
pub struct XmpFile<'xmp> {
    f: *mut ffi::CXmpFile,
    xmp: &'xmp XmpToolkit,
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
}

#[cfg(test)]
mod tests {
    use crate::XmpFile;
    use crate::XmpToolkit;

    #[test]
    fn new() {
        let xmp = XmpToolkit::new();
        let _f = XmpFile::new(&xmp);
    }
}
