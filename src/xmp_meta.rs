// TO DO: Revise API documentation to fit the Rust wrapper.

use crate::ffi;
use crate::xmp_toolkit::XmpToolkit;

#[allow(dead_code)] // because xmp is never used
pub struct XmpMeta<'xmp> {
    m: *mut ffi::CXmpMeta,
    xmp: &'xmp XmpToolkit,
}

impl<'xmp> Drop for XmpMeta<'xmp> {
    fn drop(&mut self) {
        unsafe {
            ffi::CXmpMetaDrop(self.m);
        }
    }
}

impl<'xmp> XmpMeta<'xmp> {
    /// Creates a new, empty metadata struct.
    pub fn new(xmp: &'xmp XmpToolkit) -> XmpMeta<'xmp> {
        let m = unsafe { ffi::CXmpMetaNew() };
        XmpMeta { m, xmp }
    }
}

#[cfg(test)]
mod tests {
    use crate::XmpToolkit;

    use super::*;

    #[test]
    fn new_empty() {
        let xmp = XmpToolkit::new();
        let mut _m = XmpMeta::new(&xmp);
    }
}
